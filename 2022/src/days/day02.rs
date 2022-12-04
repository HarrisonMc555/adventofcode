use lazy_static::lazy_static;
use regex::Regex;

use crate::days::{Day, Debug, Example, Part};

pub struct Day02;

impl Day for Day02 {
    fn number(&self) -> u32 {
        2
    }

    fn run(&self, part: Part, example: Example, debug: Debug) {
        let answer = match part {
            Part::Part1 => self.part1(example, debug),
            Part::Part2 => self.part2(example, debug),
        };
        println!("{}", answer);
    }
}

impl Day02 {
    fn part1(&self, example: Example, _debug: Debug) -> u32 {
        let plan = parse_plan(self.get_lines(example).iter()).unwrap();
        plan.iter().map(|(opp, mine)| score_plan(*opp, *mine)).sum()
    }

    fn part2(&self, example: Example, _debug: Debug) -> u32 {
        let plan = parse_plan2(self.get_lines(example).iter()).unwrap();
        plan.iter()
            .map(|(opp, desired_result)| (opp, get_choice_for_plan(*opp, *desired_result)))
            .map(|(opp, mine)| score_plan(*opp, mine))
            .sum()
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Choice {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum RoundResult {
    Win,
    Draw,
    Loss,
}

fn score_plan(opponent: Choice, mine: Choice) -> u32 {
    score_round(opponent, mine) + score_choice(mine)
}

fn score_round(opponent: Choice, mine: Choice) -> u32 {
    match mine.round(opponent) {
        RoundResult::Win => 6,
        RoundResult::Draw => 3,
        RoundResult::Loss => 0,
    }
}

fn score_choice(choice: Choice) -> u32 {
    match choice {
        Choice::Rock => 1,
        Choice::Paper => 2,
        Choice::Scissors => 3,
    }
}

fn get_choice_for_plan(opponent: Choice, desired_result: RoundResult) -> Choice {
    match desired_result {
        RoundResult::Win => opponent.loses_to(),
        RoundResult::Loss => opponent.beats(),
        RoundResult::Draw => opponent,
    }
}

impl Choice {
    fn round(&self, other: Choice) -> RoundResult {
        if self.beats() == other {
            RoundResult::Win
        } else if &other.beats() == self {
            RoundResult::Loss
        } else {
            RoundResult::Draw
        }
    }

    fn beats(&self) -> Self {
        match self {
            Choice::Rock => Choice::Scissors,
            Choice::Scissors => Choice::Paper,
            Choice::Paper => Choice::Rock,
        }
    }

    fn loses_to(&self) -> Self {
        match self {
            Choice::Rock => Choice::Paper,
            Choice::Scissors => Choice::Rock,
            Choice::Paper => Choice::Scissors,
        }
    }

    fn parse(string: &str) -> Option<Self> {
        Some(match string {
            "A" | "X" => Choice::Rock,
            "B" | "Y" => Choice::Paper,
            "C" | "Z" => Choice::Scissors,
            _ => return None,
        })
    }
}

impl RoundResult {
    fn parse(string: &str) -> Option<Self> {
        Some(match string {
            "X" => RoundResult::Loss,
            "Y" => RoundResult::Draw,
            "Z" => RoundResult::Win,
            _ => return None,
        })
    }
}

fn parse_plan<T, U>(lines: T) -> Option<Vec<(Choice, Choice)>>
where
    T: Iterator<Item = U>,
    U: AsRef<str>,
{
    lines.map(|s| parse_line(s.as_ref())).collect()
}

fn parse_plan2<T, U>(lines: T) -> Option<Vec<(Choice, RoundResult)>>
where
    T: Iterator<Item = U>,
    U: AsRef<str>,
{
    lines.map(|s| parse_line2(s.as_ref())).collect()
}

fn parse_line(line: &str) -> Option<(Choice, Choice)> {
    lazy_static! {
        static ref RE: Regex = Regex::new("^([ABC]) ([XYZ])$").unwrap();
    }
    let caps = RE.captures(line).unwrap();
    Some((
        Choice::parse(caps.get(1).unwrap().as_str())?,
        Choice::parse(caps.get(2).unwrap().as_str())?,
    ))
}

fn parse_line2(line: &str) -> Option<(Choice, RoundResult)> {
    lazy_static! {
        static ref RE: Regex = Regex::new("^([ABC]) ([XYZ])$").unwrap();
    }
    let caps = RE.captures(line).unwrap();
    Some((
        Choice::parse(caps.get(1).unwrap().as_str())?,
        RoundResult::parse(caps.get(2).unwrap().as_str())?,
    ))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_examples_part1() {
        assert_eq!(8, score_part1("A Y"));
        assert_eq!(1, score_part1("B X"));
        assert_eq!(6, score_part1("C Z"));
    }

    #[test]
    fn test_examples_part2() {
        assert_eq!(4, score_part2("A Y"));
        assert_eq!(1, score_part2("B X"));
        assert_eq!(7, score_part2("C Z"));
    }

    fn score_part1(line: &str) -> u32 {
        let (opp, mine) = parse_line(line).unwrap();
        score_plan(opp, mine)
    }

    fn score_part2(line: &str) -> u32 {
        let (opp, desired_result) = parse_line2(line).unwrap();
        let mine = get_choice_for_plan(opp, desired_result);
        score_plan(opp, mine)
    }
}
