use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::num::ParseIntError;
use std::ops::*;
use std::str::FromStr;

use crate::days::{Day, Debug, Example, Part};
use crate::{debug_print, debug_println};

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

    fn part2(&self, example: Example, _debug: Debug) -> isize {
        let assignments = parse_assignments(&self.read_file(example)).unwrap();
        let finder = Finder::new(&assignments);
        finder
            .find_required_value(
                GOAL_MONKEY_ID.parse().unwrap(),
                CUSTOM_MONKEY_ID.parse().unwrap(),
            )
            .unwrap()
    }
}

const GOAL_MONKEY_ID: &str = "root";
const CUSTOM_MONKEY_ID: &str = "humn";

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
struct Assignment {
    id: MonkeyID,
    job: Job,
}

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
enum Job {
    ShoutNumber(isize),
    PerformOperation(OperationInfo),
    CustomNumber,
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
                debug_println!(
                    "{} result is {} ({}) {} {} ({}) = {}",
                    id,
                    id1,
                    value1,
                    operator,
                    id2,
                    value2,
                    result
                );
                cache.insert(id, result);
                Some(result)
            }
            Job::CustomNumber => None,
        }
    }

    helper(&id_to_job, &mut cache, id)
}

struct Finder<'a> {
    id_to_job: HashMap<MonkeyID, &'a Job>,
    cache: HashMap<MonkeyID, OperationResult>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
enum OperationResult {
    Complete(isize),
    Partial(Vec<(GoalAndValueToCustom, isize)>),
}
type GoalAndValueToCustom = fn(isize, isize) -> isize;

impl<'a> Finder<'a> {
    fn new(assignments: &'a [Assignment]) -> Self {
        let id_to_job = assignments
            .iter()
            .map(|a| (a.id, &a.job))
            // .chain(std::iter::once((custom_value_id, &Job::CustomNumber)))
            .collect::<HashMap<_, _>>();
        let cache = HashMap::new();
        Finder { id_to_job, cache }
    }

    fn find_required_value(
        mut self,
        goal_id: MonkeyID,
        custom_value_id: MonkeyID,
    ) -> Option<isize> {
        debug_println!("Finding required value for {} to make {} equal", custom_value_id, goal_id);
        self.id_to_job.insert(custom_value_id, &Job::CustomNumber);
        let Job::PerformOperation(OperationInfo { id1, id2, .. }) = self.id_to_job.get(&goal_id)? else {
            debug_println!("No job found for goal: {}", goal_id);
            return None;
        };
        self.evaluate_partial(*id1);
        self.evaluate_partial(*id2);
        let (mut goal_number, stack) =
            match (self.cache.get(id1)?.clone(), self.cache.get(id2)?.clone()) {
                (OperationResult::Complete(goal_number), OperationResult::Partial(f)) => {
                    (goal_number, f)
                }
                (OperationResult::Partial(f), OperationResult::Complete(goal_number)) => {
                    (goal_number, f)
                }
                _ => return None,
            };
        for (f, value) in stack.into_iter().rev() {
            goal_number = f(goal_number, value);
        }
        Some(goal_number)
    }

    fn evaluate_partial(&mut self, id: MonkeyID) -> Option<()> {
        debug_println!("Getting value for {}", id);
        // if let Some(result) = self.cache.get(&id) {
        if self.cache.contains_key(&id) {
            debug_println!("\tAlready found value for {}", id);
            return Some(());
        }
        let result = match self.id_to_job.remove(&id)?.clone() {
            Job::ShoutNumber(number) => {
                debug_println!("{} shouts {}", id, number);
                OperationResult::Complete(number)
            }
            Job::PerformOperation(operation_info) => {
                debug_print!("{} ", id);
                self.evaluate_partial_operation(operation_info)?
            }
            Job::CustomNumber => {
                debug_println!("{} is custom number", id);
                OperationResult::Partial(Vec::new())
            }
        };
        self.cache.insert(id, result);
        Some(())
    }

    fn evaluate_partial_operation(
        &mut self,
        operation_info: OperationInfo,
    ) -> Option<OperationResult> {
        let OperationInfo { operator, id1, id2 } = operation_info;
        self.evaluate_partial(id1)?;
        self.evaluate_partial(id2)?;
        let result = match (self.cache.get(&id1)?, self.cache.get(&id2)?) {
            (OperationResult::Complete(num1), OperationResult::Complete(num2)) => {
                OperationResult::Complete(operator.apply(*num1, *num2))
            }
            (OperationResult::Complete(num1), OperationResult::Partial(stack)) => {
                let mut stack = stack.clone();
                stack.push((operator.inverse_right_unknown(), *num1));
                OperationResult::Partial(stack)
            }
            (OperationResult::Partial(stack), OperationResult::Complete(num2)) => {
                let mut stack = stack.clone();
                stack.push((operator.inverse_left_unknown(), *num2));
                OperationResult::Partial(stack)
            }
            _ => return None,
        };
        // debug_println!(
        //             "result is {} ({:?}) {} {} ({:?}) = {:?}",
        //             id1,
        //             value1,
        //             operator,
        //             id2,
        //             value2,
        //             result
        //         );
        Some(result)
    }
}

impl Operator {
    fn apply(self, value1: isize, value2: isize) -> isize {
        self.function()(value1, value2)
    }

    fn function(self) -> fn(isize, isize) -> isize {
        match self {
            Operator::Addition => Add::add,
            Operator::Subtraction => Sub::sub,
            Operator::Multiplication => Mul::mul,
            Operator::Division => Div::div,
        }
    }

    fn inverse_left_unknown(self) -> GoalAndValueToCustom {
        match self {
            Operator::Addition => Sub::sub,
            Operator::Subtraction => Add::add,
            Operator::Multiplication => Div::div,
            Operator::Division => Mul::mul,
        }
    }

    fn inverse_right_unknown(self) -> GoalAndValueToCustom {
        match self {
            Operator::Addition => Sub::sub,
            Operator::Subtraction => sub_flip,
            Operator::Multiplication => Div::div,
            Operator::Division => div_flip,
        }
    }
}

fn sub_flip<T>(lhs: T, rhs: T) -> T::Output
where
    T: Sub<T>,
{
    rhs - lhs
}

fn div_flip<T>(lhs: T, rhs: T) -> T::Output
where
    T: Div<T>,
{
    rhs / lhs
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
        assert_eq!(301, Day21.part2(Example::Example, Debug::NotDebug));
    }

    #[test]
    fn test_real_part2() {
        assert_eq!(3360561285172, Day21.part2(Example::Real, Debug::NotDebug));
    }

    fn id(s: &str) -> MonkeyID {
        s.parse().unwrap()
    }
}
