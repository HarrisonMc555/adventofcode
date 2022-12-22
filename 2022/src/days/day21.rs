use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::num::ParseIntError;
use std::str::FromStr;

use crate::days::{Day, Debug, Example, Part};
use crate::debug_println;

const DEBUG: bool = false;

pub struct Day21;

impl Day for Day21 {
    fn number(&self) -> u32 {
        21
    }

    fn run(&self, part: Part, example: Example, debug: Debug) {
        let answer = match part {
            Part::Part1 => self.part1(example, debug),
            Part::Part2 => self.part2(example, debug),
        };
        println!("{}", answer);
    }
}

impl Day21 {
    fn part1(&self, example: Example, _debug: Debug) -> isize {
        let assignments = parse_assignments(&self.read_file(example)).unwrap();
        evaluate(&assignments, GOAL_MONKEY_ID).unwrap()
    }

    fn part2(&self, _example: Example, _debug: Debug) -> isize {
        todo!()
    }
}

const GOAL_MONKEY_ID: &str = "root";

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
struct Assignment {
    id: MonkeyID,
    job: Job,
}

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
enum Job {
    ShoutNumber(isize),
    PerformOperation(OperationInfo),
}

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
struct OperationInfo {
    operator: Operator,
    id1: MonkeyID,
    id2: MonkeyID,
}

#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone)]
enum Operator {
    Addition,
    Subtraction,
    Multiplication,
    Division,
}

#[derive(Hash, Eq, PartialEq, Copy, Clone)]
struct MonkeyID([char; MONKEY_ID_NUM_CHARS]);
const MONKEY_ID_NUM_CHARS: usize = 4;

fn evaluate(assignments: &[Assignment], id: &str) -> Option<isize> {
    let id = id.parse().ok()?;
    let id_to_job = assignments
        .iter()
        .map(|a| (a.id, &a.job))
        .collect::<HashMap<_, _>>();
    let mut cache = HashMap::new();

    fn helper(
        id_to_job: &HashMap<MonkeyID, &Job>,
        cache: &mut HashMap<MonkeyID, isize>,
        id: MonkeyID,
    ) -> Option<isize> {
        debug_println!("Getting value for {}", id);
        if let Some(result) = cache.get(&id) {
            debug_println!("\tAlready found value for {}: {}", id, result);
            return Some(*result);
        }
        match id_to_job.get(&id)? {
            Job::ShoutNumber(number) => {
                debug_println!("{} shouts {}", id, number);
                cache.insert(id, *number);
                Some(*number)
            }
            Job::PerformOperation(operation_info) => {
                let OperationInfo { operator, id1, id2 } = operation_info;
                let value1 = helper(id_to_job, cache, *id1)?;
                let value2 = helper(id_to_job, cache, *id2)?;
                let result = operator.apply(value1, value2);
                debug_println!("{} result is {} ({}) {} {} ({}) = {}", id, id1, value1, operator, id2, value2, result);
                cache.insert(id, result);
                Some(result)
            }
        }
    }

    helper(&id_to_job, &mut cache, id)
}

impl Operator {
    fn apply(self, value1: isize, value2: isize) -> isize {
        self.function()(value1, value2)
    }

    fn function(self) -> fn(isize, isize) -> isize {
        match self {
            Operator::Addition => std::ops::Add::add,
            Operator::Subtraction => std::ops::Sub::sub,
            Operator::Multiplication => std::ops::Mul::mul,
            Operator::Division => std::ops::Div::div,
        }
    }
}

fn parse_assignments(s: &str) -> Result<Vec<Assignment>, ParseError> {
    s.trim().split('\n').map(|line| line.parse()).collect()
}

impl FromStr for Assignment {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let pattern = ": ";
        let (id, job) = s
            .split_once(pattern)
            .ok_or_else(|| parse_error::<Assignment>(s))?;
        let id = id.parse()?;
        let job = job.parse()?;
        Ok(Assignment { id, job })
    }
}

impl FromStr for MonkeyID {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let letters = take_exact(s.chars()).ok_or_else(|| parse_error::<MonkeyID>(s))?;
        Ok(MonkeyID(letters))
    }
}

impl FromStr for Job {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.split_whitespace();
        let err = || parse_error::<ParseError>(s);
        let first = iter.next().ok_or_else(err)?;
        Ok(match (iter.next(), iter.next()) {
            (None, None) => Job::ShoutNumber(first.parse()?),
            (Some(operator), Some(id2)) => {
                let id1 = first.parse()?;
                let operator = operator.parse()?;
                let id2 = id2.parse()?;
                Job::PerformOperation(OperationInfo { operator, id1, id2 })
            }
            _ => return Err(err()),
        })
    }
}

impl FromStr for Operator {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "+" => Operator::Addition,
            "-" => Operator::Subtraction,
            "*" => Operator::Multiplication,
            "/" => Operator::Division,
            _ => return Err(parse_error::<Operator>(s)),
        })
    }
}

#[derive(Debug, Hash, Clone, Eq, PartialEq)]
struct ParseError {
    expected: String,
    actual: String,
}

fn parse_error<T>(s: &str) -> ParseError {
    ParseError {
        expected: std::any::type_name::<T>().to_string(),
        actual: s.to_string(),
    }
}

impl From<ParseIntError> for ParseError {
    fn from(error: ParseIntError) -> Self {
        ParseError {
            expected: "integer".to_string(),
            actual: error.to_string(),
        }
    }
}

fn take_exact<I: Iterator, const N: usize>(mut iter: I) -> Option<[I::Item; N]> {
    let mut result = Vec::with_capacity(N);
    for _ in 0..N {
        result.push(iter.next()?);
    }
    if iter.next().is_some() {
        return None;
    }
    result.try_into().ok()
}

impl Display for MonkeyID {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for c in self.0 {
            write!(f, "{}", c)?;
        }
        Ok(())
    }
}

impl std::fmt::Debug for MonkeyID {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "\"{}\"", self)
    }
}

impl Display for Operator {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            Operator::Addition => '+',
            Operator::Subtraction => '-',
            Operator::Multiplication => '*',
            Operator::Division => '/',
        };
        write!(f, "{}", c)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse() {
        let actual = "root: pppw + sjmn".parse().unwrap();
        let expected = Assignment {
            id: id("root"),
            job: Job::PerformOperation(OperationInfo {
                operator: Operator::Addition,
                id1: id("pppw"),
                id2: id("sjmn"),
            }),
        };
        assert_eq!(expected, actual);

        let actual = "sjmn: drzm * dbpl".parse().unwrap();
        let expected = Assignment {
            id: id("sjmn"),
            job: Job::PerformOperation(OperationInfo {
                operator: Operator::Multiplication,
                id1: id("drzm"),
                id2: id("dbpl"),
            }),
        };
        assert_eq!(expected, actual);

        assert!(parse_assignments(include_str!("../../static/example21.txt")).is_ok());
    }

    #[test]
    fn test_examples_part1() {
        assert_eq!(152, Day21.part1(Example::Example, Debug::NotDebug));
    }

    #[test]
    fn test_real_part1() {
        assert_eq!(152479825094094, Day21.part1(Example::Real, Debug::NotDebug));
    }

    #[test]
    fn test_examples_part2() {
        // assert_eq!(0, Day21.part2(Example::Example, Debug::NotDebug));
    }

    #[test]
    fn test_real_part2() {
        // assert_eq!(0, Day21.part2(Example::Real, Debug::NotDebug));
    }

    fn id(s: &str) -> MonkeyID {
        s.parse().unwrap()
    }
}
