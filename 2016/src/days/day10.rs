use crate::days::{Day, Debug, Example, Part};
use lazy_static::lazy_static;
use regex::{Captures, Regex};
use std::collections::{HashMap, VecDeque};

const DEBUG: bool = false;
#[macro_export]
macro_rules! debug_println {
    ($($tts:tt)*) => {
        if (DEBUG) {
            println!($($tts)*);
        }
    }
}

pub struct Day10;

impl Day for Day10 {
    fn number(&self) -> u32 {
        10
    }

    fn run(&self, part: Part, example: Example, debug: Debug) {
        let answer = match part {
            Part::Part1 => self.part1(example, debug),
            Part::Part2 => self.part2(example, debug),
        };
        println!("{}", answer);
    }
}

impl Day10 {
    fn part1(&self, example: Example, _debug: Debug) -> u32 {
        let instructions = parse(&self.read_file(example)).unwrap();
        let (compare_value_1, compare_value_2) = match example {
            Example::Real => (&COMPARE_VALUE_1, &COMPARE_VALUE_2),
            Example::Example => (&EXAMPLE_COMPARE_VALUE_1, &EXAMPLE_COMPARE_VALUE_2),
        };
        find_bot_that_compares_values(&instructions, compare_value_1, compare_value_2).unwrap().0
    }

    fn part2(&self, _example: Example, _debug: Debug) -> u32 {
        todo!()
    }
}

const COMPARE_VALUE_1: ValueID = ValueID(61);
const COMPARE_VALUE_2: ValueID = ValueID(17);

const EXAMPLE_COMPARE_VALUE_1: ValueID = ValueID(5);
const EXAMPLE_COMPARE_VALUE_2: ValueID = ValueID(2);

#[derive(Debug)]
struct Instructions {
    initials: Vec<Initial>,
    gives: Vec<Give>,
}

#[derive(Debug, Eq, PartialEq)]
enum Instruction {
    Initial(Initial),
    Give(Give),
}

#[derive(Debug, Eq, PartialEq)]
struct Initial {
    value: ValueID,
    bot: BotID,
}

#[derive(Debug, Eq, PartialEq)]
struct Give {
    bot: BotID,
    low: Destination,
    high: Destination,
}

#[derive(Debug, Eq, PartialEq)]
enum Destination {
    Bot(BotID),
    Output(OutputID),
}

#[derive(Debug, Hash, Eq, PartialEq)]
struct BotID(u32);
#[derive(Debug, Hash, Eq, PartialEq, Ord, PartialOrd)]
struct ValueID(u32);
#[derive(Debug, Hash, Eq, PartialEq)]
struct OutputID(u32);

fn find_bot_that_compares_values<'a>(
    instructions: &'a Instructions,
    compare_value_1: &ValueID,
    compare_value_2: &ValueID,
) -> Option<&'a BotID> {
    let (compare_value_low, compare_value_high) = get_low_high(&compare_value_1, &compare_value_2);
    let bot_to_give = instructions
        .gives
        .iter()
        .map(|give| (&give.bot, give))
        .collect::<HashMap<_, _>>();
    let mut dirty_bot_ids = instructions
        .initials
        .iter()
        .map(|initial| &initial.bot)
        .collect::<VecDeque<_>>();
    let mut bot_to_values = HashMap::<&BotID, VecDeque<&ValueID>>::new();
    for initial in instructions.initials.iter() {
        bot_to_values
            .entry(&initial.bot)
            .or_insert_with(VecDeque::new)
            .push_back(&initial.value);
    }
    let mut output_to_values = HashMap::<&OutputID, VecDeque<&ValueID>>::new();

    while let Some(bot_id) = dirty_bot_ids.pop_front() {
        debug_println!("Processing bot {}", bot_id.0);
        debug_println!("\t{} dirty bot IDs", dirty_bot_ids.len());
        let Some(values) = bot_to_values.get_mut(&bot_id) else {
            debug_println!("\tNo values for bot {}", bot_id.0);
            continue;
        };
        if values.len() < 2 {
            debug_println!("\tBot {} only had {} values", bot_id.0, values.len());
            continue;
        }
        let (Some(value1), Some(value2)) = (values.pop_front(), values.pop_front()) else {
            panic!("Length was less than 2 but couldn't pop 2 values");
        };
        let (low, high) = get_low_high(value1, value2);

        if low == compare_value_low && high == compare_value_high {
            return Some(bot_id);
        }

        let Some(give) = bot_to_give.get(&bot_id) else {
            debug_println!("\tBot {} doesn't know who to give things to!", bot_id.0);
            continue;
        };

        match &give.low {
            Destination::Bot(low_bot_id) => {
                debug_println!("\tGive to bot {}", low_bot_id.0);
                bot_to_values
                    .entry(&low_bot_id)
                    .or_insert_with(VecDeque::new)
                    .push_back(low);
                dirty_bot_ids.push_back(&low_bot_id);
            }
            Destination::Output(low_output_id) => {
                debug_println!("\tGive to output {}", low_output_id.0);
                output_to_values
                    .entry(&low_output_id)
                    .or_insert_with(VecDeque::new)
                    .push_back(low);
            }
        }
        match &give.high {
            Destination::Bot(high_bot_id) => {
                debug_println!("\tGive to bot {}", high_bot_id.0);
                bot_to_values
                    .entry(&high_bot_id)
                    .or_insert_with(VecDeque::new)
                    .push_back(high);
                dirty_bot_ids.push_back(&high_bot_id);
            }
            Destination::Output(high_output_id) => {
                debug_println!("\tGive to output {}", high_output_id.0);
                output_to_values
                    .entry(&high_output_id)
                    .or_insert_with(VecDeque::new)
                    .push_back(high);
            }
        }
    }

    None
}

fn get_low_high<'a>(value1: &'a ValueID, value2: &'a ValueID) -> (&'a ValueID, &'a ValueID) {
    let mut temp = [value1, value2];
    temp.sort();
    let [low, high] = temp;
    (low, high)
}

fn parse(text: &str) -> Option<Instructions> {
    let mut initials = Vec::new();
    let mut gives = Vec::new();
    for instruction in text.trim().split('\n').map(Instruction::parse) {
        match instruction? {
            Instruction::Initial(initial) => initials.push(initial),
            Instruction::Give(give) => gives.push(give),
        }
    }
    Some(Instructions { initials, gives })
}

impl Instruction {
    fn parse(line: &str) -> Option<Self> {
        lazy_static! {
            static ref INITIAL: Regex = Regex::new(r"^value (\d+) goes to bot (\d+)$").unwrap();
            static ref GIVE: Regex =
                Regex::new(r"^bot (\d+) gives low to (.+) and high to (.+)$").unwrap();
        }

        fn parse_num(caps: &Captures, group: usize) -> Option<u32> {
            caps.get(group).unwrap().as_str().parse().ok()
        }
        if let Some(caps) = INITIAL.captures(line) {
            let value = ValueID(parse_num(&caps, 1)?);
            let bot = BotID(parse_num(&caps, 2)?);
            Some(Initial { value, bot }.into())
        } else if let Some(caps) = GIVE.captures(line) {
            let bot = BotID(parse_num(&caps, 1)?);
            let low = caps.get(2).unwrap().as_str();
            let high = caps.get(3).unwrap().as_str();
            let low = Destination::parse(low)?;
            let high = Destination::parse(high)?;
            Some(Give { bot, low, high }.into())
        } else {
            None
        }
    }
}

impl Destination {
    fn parse(text: &str) -> Option<Self> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^(bot|output) (\d+)$").unwrap();
        }

        let caps = RE.captures(text)?;
        let type_str = caps.get(1).unwrap().as_str();
        let id = caps.get(2).unwrap().as_str().parse().ok()?;
        Some(match type_str {
            "bot" => Destination::Bot(BotID(id)),
            "output" => Destination::Output(OutputID(id)),
            _ => return None,
        })
    }
}

impl From<Initial> for Instruction {
    fn from(initial: Initial) -> Self {
        Instruction::Initial(initial)
    }
}

impl From<Give> for Instruction {
    fn from(give: Give) -> Self {
        Instruction::Give(give)
    }
}

impl From<u32> for BotID {
    fn from(id: u32) -> Self {
        BotID(id)
    }
}

impl From<u32> for ValueID {
    fn from(id: u32) -> Self {
        ValueID(id)
    }
}

impl From<u32> for OutputID {
    fn from(id: u32) -> Self {
        OutputID(id)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse() {
        use super::Destination::*;

        let text = include_str!("../../static/example10.txt");
        let instructions = parse(text).unwrap();
        let mut initials_iter = instructions.initials.into_iter();
        let mut gives_iter = instructions.gives.into_iter();

        let instruction = Initial {
            value: 5.into(),
            bot: 2.into(),
        };
        assert_eq!(instruction, initials_iter.next().unwrap());

        let instruction = Give {
            bot: 2.into(),
            low: Bot(1.into()),
            high: Bot(0.into()),
        };
        assert_eq!(instruction, gives_iter.next().unwrap());

        let instruction = Initial {
            value: 3.into(),
            bot: 1.into(),
        };
        assert_eq!(instruction, initials_iter.next().unwrap());

        let instruction = Give {
            bot: 1.into(),
            low: Output(1.into()),
            high: Bot(0.into()),
        };
        assert_eq!(instruction, gives_iter.next().unwrap());

        let instruction = Give {
            bot: 0.into(),
            low: Output(2.into()),
            high: Output(0.into()),
        };
        assert_eq!(instruction, gives_iter.next().unwrap());

        let instruction = Initial {
            value: 2.into(),
            bot: 2.into(),
        };
        assert_eq!(instruction, initials_iter.next().unwrap());

        assert!(initials_iter.next().is_none());
        assert!(gives_iter.next().is_none());
    }

    #[test]
    fn test_examples_part1() {
        let instructions = parse(include_str!("../../static/example10.txt")).unwrap();
        let actual = find_bot_that_compares_values(&instructions, &EXAMPLE_COMPARE_VALUE_1, &EXAMPLE_COMPARE_VALUE_2);
        let expected = BotID(2);
        assert_eq!(Some(&expected), actual);
    }

    #[test]
    fn test_real_part1() {
        assert_eq!(147, Day10.part1(Example::Real, Debug::NotDebug));
    }

    #[test]
    fn test_examples_part2() {}

    #[test]
    fn test_real_part2() {
        // assert_eq!(0, Day10.part2(Example::Real, Debug::NotDebug));
    }
}
