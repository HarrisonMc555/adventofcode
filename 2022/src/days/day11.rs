use crate::days::{Day, Debug, Example, Part};
use crate::debug_println;
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::VecDeque;

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
        calc_monkey_business(&mut monkeys, NUM_ROUNDS).unwrap()
    }

    fn part2(&self, _example: Example, _debug: Debug) -> usize {
        todo!()
    }
}

const NUM_ROUNDS: usize = 20;

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
    divisible_by: u32,
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
    Literal(u32),
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct MonkeyID(usize);
#[derive(Debug, Eq, PartialEq)]
struct Item {
    id: String,
    worry_level: u32,
}

fn calc_monkey_business(monkeys: &mut [Monkey], num_rounds: usize) -> Option<usize> {
    for (index, monkey) in monkeys.iter().enumerate() {
        if monkey.id.0 != index {
            debug_println!("Monkey ID {} did not match index {}", monkey.id.0, index);
            return None;
        }
    }
    for _round_index in 0..num_rounds {
        run_round(monkeys);
    }
    let mut inspection_counts = monkeys
        .iter()
        .map(|monkey| monkey.inspection_count)
        .collect::<Vec<_>>();
    inspection_counts.sort_unstable_by(|a, b| b.cmp(a));
    let [count1, count2, ..] = inspection_counts.as_slice() else {
        return None;
    };
    Some(count1 * count2)
}

fn run_round(monkeys: &mut [Monkey]) {
    for monkey_index in 0..monkeys.len() {
        run_turn(monkeys, monkey_index);
    }
}

#[allow(unreachable_code)]
fn run_turn(monkeys: &mut [Monkey], monkey_index: usize) {
    debug_println!("Monkey {}:", monkey_index);
    loop {
        let monkey = &mut monkeys[monkey_index];
        let Some((item, dest_monkey_id)) = monkey.inspect() else {
            break;
        };
        monkeys[dest_monkey_id.0].throw_item_to(item);
    }
}

impl Monkey {
    fn inspect(&mut self) -> Option<(Item, MonkeyID)> {
        let mut item = self.items.pop_front()?;
        let dest_monkey_id = self.test.inspect(&mut item);
        self.inspection_count += 1;
        Some((item, dest_monkey_id))
    }

    fn throw_item_to(&mut self, item: Item) {
        self.items.push_back(item);
    }
}

impl Test {
    fn inspect(&self, item: &mut Item) -> MonkeyID {
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
        let final_worry_level = new / 3;
        debug_println!(
            "    Monkey gets bored with item. Worry level is divided by 3 to {}.",
            final_worry_level,
        );
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
    fn new(id: String, worry_level: u32) -> Self {
        Item {
            id,
            worry_level,
        }
    }

    fn change_worry_level(&mut self, worry_level: u32) {
        self.worry_level = worry_level;
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
        let parse_u32 = |group| get_cap(group).parse::<u32>().ok();
        let parse_monkey_id = |group| get_cap(group).parse::<usize>().ok().map(MonkeyID);
        let id = parse_monkey_id(1)?;
        let items = Monkey::parse_items(get_cap(2), item_id_generator)?;
        let operation = Operation::parse(get_cap(3))?;
        let amount = Amount::parse(get_cap(4))?;
        let divisible_by = parse_u32(5)?;
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

fn is_divisible_by(dividend: u32, divisor: u32) -> bool {
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
    fn test_examples_part2() {}

    #[test]
    fn test_real_part2() {
        // assert_eq!(0, Day11.part2(Example::Real, Debug::NotDebug));
    }

    fn create_items(
        worry_levels: &[u32],
        item_id_generator: &mut ItemIdGenerator,
    ) -> VecDeque<Item> {
        worry_levels
            .iter()
            .copied()
            .map(|worry_level| Item::new(item_id_generator.next(), worry_level))
            .collect()
    }
}
