use std::collections::{HashSet, VecDeque};

use lazy_static::lazy_static;
use regex::Regex;

use crate::days::{Day, Debug, Example, Part};
use crate::debug_println;

pub struct Day11;

const DEBUG: bool = false;

impl Day for Day11 {
    fn number(&self) -> u32 {
        11
    }

    fn run(&self, part: Part, example: Example, debug: Debug) {
        let answer = match part {
            Part::Part1 => self.part1(example, debug),
            Part::Part2 => self.part2(example, debug),
        };
        println!("{}", answer);
    }
}

impl Day11 {
    fn part1(&self, example: Example, _debug: Debug) -> usize {
        let mut monkeys = Monkey::parse_monkeys(&self.read_file(example)).unwrap();
        calc_monkey_business(&mut monkeys, NUM_ROUNDS, InspectionResponse::Relieved).unwrap()
    }

    fn part2(&self, example: Example, _debug: Debug) -> usize {
        let mut monkeys = Monkey::parse_monkeys(&self.read_file(example)).unwrap();
        calc_monkey_business(&mut monkeys, NUM_ROUNDS2, InspectionResponse::Unaltered).unwrap()
    }
}

const NUM_ROUNDS: usize = 20;
const NUM_ROUNDS2: usize = 10_000;

#[derive(Debug, Eq, PartialEq)]
struct Monkey {
    id: MonkeyID,
    items: VecDeque<Item>,
    test: Test,
    inspection_count: usize,
}

#[derive(Debug, Eq, PartialEq)]
struct Test {
    operation: Operation,
    amount: Amount,
    divisible_by: u64,
    true_dest: MonkeyID,
    false_dest: MonkeyID,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Operation {
    Addition,
    Multiplication,
}

#[derive(Debug, Clone, Eq, PartialEq)]
enum Amount {
    Old,
    Literal(u64),
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
struct MonkeyID(usize);

#[derive(Debug, Clone, Eq, PartialEq)]
struct Item {
    id: String,
    worry_level: u64,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum InspectionResponse {
    Relieved,
    Unaltered,
}

type Rounds = Vec<Vec<MonkeyID>>;

fn calc_monkey_business(
    monkeys: &mut [Monkey],
    num_rounds: usize,
    inspection_response: InspectionResponse,
) -> Option<usize> {
    for (index, monkey) in monkeys.iter().enumerate() {
        if monkey.id.0 != index {
            debug_println!("Monkey ID {} did not match index {}", monkey.id.0, index);
            return None;
        }
    }

    // Get into "stable" state.
    run_round(monkeys, inspection_response);
    let num_rounds = num_rounds - 1;

    let mut inspection_counts = monkeys
        .iter()
        .map(|monkey| monkey.inspection_count)
        .collect::<Vec<_>>();
    for item in monkeys.iter().flat_map(|monkey| monkey.items.iter()) {
        let (prefix, cycle) = item.clone().find_cycle(monkeys, inspection_response)?;
        debug_println!();
        debug_println!("= Finished finding cycle for item {} =", item.id);
        debug_println!("Prefix length: {}", prefix.len());
        debug_println!("Cycle length : {}", cycle.len());
        debug_println!(
            "Before updating with prefix counts, inspection counts: {:?}",
            inspection_counts
        );
        for (round_index, round) in prefix.iter().enumerate().take(num_rounds) {
            debug_println!("  Round {}", round_index);
            for monkey_id in round.iter() {
                debug_println!("    Inspected by monkey {}", monkey_id.0);
                *inspection_counts.get_mut(monkey_id.0)? += 1;
            }
        }
        debug_println!(
            "After updating with prefix counts, inspection counts: {:?}",
            inspection_counts
        );
        if num_rounds < prefix.len() {
            debug_println!("Prefix took up entire rounds, no need to process cycle");
            continue;
        }
        let num_rounds_after_prefix = num_rounds - prefix.len();
        let num_complete_cycles = num_rounds_after_prefix / cycle.len();
        debug_println!("Number of rounds after prefix: {}", num_rounds_after_prefix);
        debug_println!("Number of completed cycles:    {}", num_complete_cycles);
        for (round_index, round) in cycle.iter().enumerate() {
            debug_println!("  Cycle round: {}", round_index);
            for monkey_id in round.iter() {
                debug_println!(
                    "    Inspected {} times by monkey {}",
                    num_complete_cycles,
                    monkey_id.0
                );
                *inspection_counts.get_mut(monkey_id.0)? += num_complete_cycles;
            }
        }
        debug_println!("After cycles, inspection counts: {:?}", inspection_counts);
        let num_remaining_rounds = num_rounds_after_prefix % cycle.len();
        debug_println!(
            "After complete cycles, there were {} rounds left over",
            num_remaining_rounds
        );
        for (round_index, round) in cycle.iter().enumerate().take(num_remaining_rounds) {
            debug_println!("  Remaining cycle round: {}", round_index);
            for monkey_id in round.iter() {
                debug_println!("    Inspected by monkey {}", monkey_id.0);
                *inspection_counts.get_mut(monkey_id.0)? += 1;
            }
        }
        debug_println!(
            "After remaining cycles, inspection counts: {:?}",
            inspection_counts
        );
        debug_println!("= Finished processing item {} =", item.id);
    }

    debug_println!("Final inspection counts: {:?}", inspection_counts);
    inspection_counts.sort_unstable_by(|a, b| b.cmp(a));
    let [count1, count2, ..] = inspection_counts.as_slice() else {
        return None;
    };
    debug_println!("Top 2 inspection counts: {} and {}", count1, count2);
    Some(count1 * count2)
}

fn run_round(monkeys: &mut [Monkey], inspection_response: InspectionResponse) {
    for monkey_index in 0..monkeys.len() {
        run_turn(monkeys, monkey_index, inspection_response);
    }
}

#[allow(unreachable_code)]
fn run_turn(monkeys: &mut [Monkey], monkey_index: usize, inspection_response: InspectionResponse) {
    debug_println!("Monkey {}:", monkey_index);
    loop {
        let monkey = &mut monkeys[monkey_index];
        let Some((item, dest_monkey_id)) = monkey.inspect(inspection_response) else {
            break;
        };
        monkeys[dest_monkey_id.0].throw_item_to(item);
    }
}

impl Monkey {
    fn inspect(&mut self, inspection_response: InspectionResponse) -> Option<(Item, MonkeyID)> {
        let mut item = self.items.pop_front()?;
        let dest_monkey_id = self.test.inspect(&mut item, inspection_response);
        self.inspection_count += 1;
        Some((item, dest_monkey_id))
    }

    fn throw_item_to(&mut self, item: Item) {
        self.items.push_back(item);
    }
}

impl Test {
    fn inspect(&self, item: &mut Item, inspection_response: InspectionResponse) -> MonkeyID {
        let old_worry_level = item.worry_level;
        debug_println!(
            "  Monkey inspects an item with a worry level of {} (item {}).",
            old_worry_level,
            item.id
        );
        let operand = match self.amount {
            Amount::Old => old_worry_level,
            Amount::Literal(literal) => literal,
        };
        let new = match self.operation {
            Operation::Addition => old_worry_level + operand,
            Operation::Multiplication => old_worry_level * operand,
        };
        debug_println!(
            "    Worry level is {} by {} to {}.",
            self.operation.verb(),
            operand,
            new,
        );
        let final_worry_level = match inspection_response {
            InspectionResponse::Relieved => {
                let final_worry_level = new / 3;
                debug_println!(
                    "    Monkey gets bored with item. Worry level is divided by 3 to {}.",
                    final_worry_level
                );
                final_worry_level
            }
            InspectionResponse::Unaltered => {
                debug_println!("    Monkey gets bored with item. Worry level is unaltered.");
                new
            }
        };
        let is_divisible = is_divisible_by(final_worry_level, self.divisible_by);
        debug_println!(
            "    Current worry level {} divisible by {}.",
            if is_divisible { "is" } else { "is not " },
            self.divisible_by,
        );
        let dest_monkey_id = if is_divisible {
            &self.true_dest
        } else {
            &self.false_dest
        };
        debug_println!(
            "    Item with worry level {} is thrown to monkey {}.",
            final_worry_level,
            dest_monkey_id.0
        );
        item.change_worry_level(final_worry_level);
        dest_monkey_id.clone()
    }
}

impl Item {
    fn new(id: String, worry_level: u64) -> Self {
        Item { id, worry_level }
    }

    fn change_worry_level(&mut self, worry_level: u64) {
        self.worry_level = worry_level;
    }

    fn find_cycle(
        &mut self,
        monkeys: &[Monkey],
        inspection_response: InspectionResponse,
    ) -> Option<(Rounds, Rounds)> {
        debug_println!("== Finding cycle for item {} ==", self.id);
        let lcm = monkeys
            .iter()
            .map(|monkey| monkey.test.divisible_by)
            .product::<u64>();
        debug_println!("Least common multiple (lcm): {}", lcm);
        let mut seen = HashSet::new();
        let mut cur_monkey = monkeys.iter().find(|monkey| monkey.items.contains(self))?;
        debug_println!("Starting monkey: {}", cur_monkey.id.0);
        let mut cycle: Vec<Vec<(MonkeyID, u64)>> = Vec::new();
        let mut cur_round = Vec::new();
        loop {
            debug_println!("  Current monkey: {}", cur_monkey.id.0);
            self.worry_level %= lcm;
            debug_println!("  Current worry level: {}", self.worry_level);
            let state = (cur_monkey.id.clone(), self.worry_level);
            if seen.contains(&state) {
                debug_println!("We've already seen {:?}", state);
                debug_println!("Total cycle length: {}", cycle.len());

                fn cycle_to_rounds<T, U>(cycle_iter: T) -> Rounds
                where
                    T: AsRef<[U]>,
                    U: AsRef<[(MonkeyID, u64)]>,
                {
                    cycle_iter
                        .as_ref()
                        .iter()
                        .map(|round| {
                            round
                                .as_ref()
                                .iter()
                                .map(|(monkey_id, _)| monkey_id.clone())
                                .collect()
                        })
                        .collect()
                }

                let (split_index, sub_index) = cycle
                    .iter()
                    .enumerate()
                    .flat_map(|(round_index, round)| {
                        round
                            .iter()
                            .enumerate()
                            .find(|(_, s)| **s == state)
                            .map(|(sub_index, _)| (round_index, sub_index))
                    })
                    .next()?;

                if cur_round.is_empty() {
                    debug_println!("Cycle: {:?}", cycle);
                    let (prefix, cycle) = cycle.split_at(split_index);
                    let prefix = cycle_to_rounds(prefix.iter());
                    let cycle = cycle_to_rounds(cycle.iter());
                    debug_println!("Prefix: {:?}", prefix);
                    debug_println!("Cycle : {:?}", cycle);
                    return Some((prefix, cycle));
                } else {
                    debug_println!("=== NOT EMPTY ===");
                    debug_println!("Cycle: {:?}", cycle);
                    debug_println!("Duplicate state: {:?}", state);
                    debug_println!("Current round: {:?}", cur_round);
                    let prefix = cycle_to_rounds(&cycle);
                    let mut new_round = cur_round.to_vec();
                    let old_round = &cycle[split_index];
                    new_round.extend_from_slice(&old_round[sub_index..]);
                    let mut new_cycle = vec![new_round];
                    new_cycle.extend_from_slice(&cycle[split_index + 1..]);
                    let cycle = cycle_to_rounds(new_cycle);
                    return Some((prefix, cycle));
                }
            }
            seen.insert(state.clone());
            cur_round.push(state.clone());
            let next_monkey_id = cur_monkey.test.inspect(self, inspection_response);
            if next_monkey_id < cur_monkey.id {
                cycle.push(cur_round.clone());
                cur_round.clear();
            }
            cur_monkey = monkeys.get(next_monkey_id.0)?;
            debug_println!(
                "  Seen: {:?}",
                seen.iter()
                    .map(|(monkey_id, worry_level)| (monkey_id.0, worry_level))
                    .collect::<HashSet<_>>()
            );
            debug_println!(
                "  Current cycle: {:?}",
                cycle
                    .iter()
                    .map(|round| round
                        .iter()
                        .map(|(monkey_id, worry_level)| (monkey_id.0, worry_level))
                        .collect::<Vec<_>>())
                    .collect::<Vec<_>>()
            );
            debug_println!();
        }
    }
}

impl Monkey {
    fn parse_monkeys(text: &str) -> Option<Vec<Self>> {
        let mut item_id_generator = ItemIdGenerator::new();
        let mut monkeys = Vec::new();
        for section in text.split("\n\n") {
            monkeys.push(Monkey::parse(section, &mut item_id_generator)?);
        }
        Some(monkeys)
    }

    fn parse(text: &str, item_id_generator: &mut ItemIdGenerator) -> Option<Self> {
        lazy_static! {
            static ref RE: Regex = Regex::new(
                "Monkey (\\d+):\\s*\
                 Starting items: (.*)\\s*\
                 Operation: new = old ([*+]) (old|\\d+)\\s*\
                 Test: divisible by (\\d+)\\s*\
                 If true: throw to monkey (\\d+)\\s*\
                 If false: throw to monkey (\\d+)\
                 "
            )
            .unwrap();
        }

        let caps = RE.captures(text)?;
        let get_cap = |group| caps.get(group).unwrap().as_str();
        let parse_u64 = |group| get_cap(group).parse::<u64>().ok();
        let parse_monkey_id = |group| get_cap(group).parse::<usize>().ok().map(MonkeyID);
        let id = parse_monkey_id(1)?;
        let items = Monkey::parse_items(get_cap(2), item_id_generator)?;
        let operation = Operation::parse(get_cap(3))?;
        let amount = Amount::parse(get_cap(4))?;
        let divisible_by = parse_u64(5)?;
        let true_dest = parse_monkey_id(6)?;
        let false_dest = parse_monkey_id(7)?;

        let test = Test {
            operation,
            amount,
            divisible_by,
            true_dest,
            false_dest,
        };
        Some(Monkey {
            id,
            items,
            test,
            inspection_count: 0,
        })
    }

    fn parse_items(text: &str, item_id_generator: &mut ItemIdGenerator) -> Option<VecDeque<Item>> {
        text.trim()
            .split(", ")
            .map(|item_text| Item::parse(item_text, item_id_generator))
            .collect()
    }
}

impl Item {
    fn parse(text: &str, item_id_generator: &mut ItemIdGenerator) -> Option<Self> {
        let id = item_id_generator.next();
        let worry_level = text.parse().ok()?;
        Some(Item::new(id, worry_level))
    }
}

impl Operation {
    fn parse(text: &str) -> Option<Self> {
        Some(match text {
            "+" => Operation::Addition,
            "*" => Operation::Multiplication,
            _ => return None,
        })
    }

    fn verb(&self) -> &'static str {
        match self {
            Operation::Addition => "added",
            Operation::Multiplication => "multiplied",
        }
    }
}

impl Amount {
    fn parse(text: &str) -> Option<Self> {
        if text == "old" {
            return Some(Amount::Old);
        }
        let literal = text.parse().ok()?;
        Some(Amount::Literal(literal))
    }
}

fn is_divisible_by(dividend: u64, divisor: u64) -> bool {
    dividend % divisor == 0
}

struct ItemIdGenerator(Vec<char>);

impl ItemIdGenerator {
    fn new() -> Self {
        ItemIdGenerator(Vec::new())
    }

    fn next(&mut self) -> String {
        const NUM_LETTERS: usize = 26;
        for letter in self.0.iter_mut() {
            if *letter == 'z' {
                *letter = 'a';
            } else {
                *letter = (*letter as u8 + 1) as char;
                return self.get_id();
            }
        }
        self.0.push('a');
        self.get_id()
    }

    fn get_id(&self) -> String {
        self.0.iter().rev().collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_item_id_generator() {
        let gen = &mut ItemIdGenerator::new();
        assert_eq!("a", gen.next());
        assert_eq!("b", gen.next());
        assert_eq!("c", gen.next());
        assert_eq!("d", gen.next());
        assert_eq!("e", gen.next());
        while gen.next() != "z" {}
        assert_eq!("aa", gen.next());
        assert_eq!("ab", gen.next());
        assert_eq!("ac", gen.next());
    }

    #[test]
    fn test_parse() {
        let text = include_str!("../../static/example11.txt");
        let monkeys = Monkey::parse_monkeys(text).unwrap();
        let gen = &mut ItemIdGenerator::new();

        let actual0 = &monkeys[0];
        let expected0 = Monkey {
            id: MonkeyID(0),
            items: create_items(&[79, 98], gen),
            test: Test {
                operation: Operation::Multiplication,
                amount: Amount::Literal(19),
                divisible_by: 23,
                true_dest: MonkeyID(2),
                false_dest: MonkeyID(3),
            },
            inspection_count: 0,
        };
        assert_eq!(&expected0, actual0);

        let actual1 = &monkeys[1];
        let expected1 = Monkey {
            id: MonkeyID(1),
            items: create_items(&[54, 65, 75, 74], gen),
            test: Test {
                operation: Operation::Addition,
                amount: Amount::Literal(6),
                divisible_by: 19,
                true_dest: MonkeyID(2),
                false_dest: MonkeyID(0),
            },
            inspection_count: 0,
        };
        assert_eq!(&expected1, actual1);

        let actual2 = &monkeys[2];
        let expected2 = Monkey {
            id: MonkeyID(2),
            items: create_items(&[79, 60, 97], gen),
            test: Test {
                operation: Operation::Multiplication,
                amount: Amount::Old,
                divisible_by: 13,
                true_dest: MonkeyID(1),
                false_dest: MonkeyID(3),
            },
            inspection_count: 0,
        };
        assert_eq!(&expected2, actual2);

        let actual3 = &monkeys[3];
        let expected3 = Monkey {
            id: MonkeyID(3),
            items: create_items(&[74], gen),
            test: Test {
                operation: Operation::Addition,
                amount: Amount::Literal(3),
                divisible_by: 17,
                true_dest: MonkeyID(0),
                false_dest: MonkeyID(1),
            },
            inspection_count: 0,
        };
        assert_eq!(&expected3, actual3);

        assert_eq!(4, monkeys.len());
    }

    #[test]
    fn test_examples_part1() {
        assert_eq!(10605, Day11.part1(Example::Example, Debug::NotDebug));
    }

    #[test]
    fn test_real_part1() {
        assert_eq!(61005, Day11.part1(Example::Real, Debug::NotDebug));
    }

    #[test]
    fn test_examples_part2() {
        assert_eq!(2713310158, Day11.part2(Example::Example, Debug::NotDebug));
    }

    #[test]
    fn test_real_part2() {
        assert_eq!(20567144694, Day11.part2(Example::Real, Debug::NotDebug));
    }

    fn create_items(
        worry_levels: &[u64],
        item_id_generator: &mut ItemIdGenerator,
    ) -> VecDeque<Item> {
        worry_levels
            .iter()
            .copied()
            .map(|worry_level| Item::new(item_id_generator.next(), worry_level))
            .collect()
    }
}
