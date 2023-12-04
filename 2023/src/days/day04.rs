use crate::days::{Day, Debug, Example, IResult, Part};
use nom::bytes::complete::tag;
use nom::character::complete as character;
use nom::multi::separated_list1;
use nom::sequence::tuple;
use std::collections::HashSet;

pub struct Day04;

impl Day for Day04 {
    fn number(&self) -> u32 {
        4
    }

    fn part1(&self, example: Example, _debug: Debug) -> String {
        self.get_lines(Part::Part1, example)
            .into_iter()
            .map(|line| parse_card(&line).unwrap())
            .map(|card| card.score())
            .sum::<u32>()
            .to_string()
    }

    fn part2(&self, example: Example, _debug: Debug) -> String {
        let cards = self.get_lines(Part::Part2, example)
            .into_iter()
            .map(|line| parse_card(&line).unwrap())
            .collect::<Vec<_>>();
        score_part2(&cards).unwrap().to_string()
    }
}

#[derive(Debug, Hash, Eq, PartialEq, Ord, PartialOrd)]
struct Card {
    id: u32,
    winning_numbers: Vec<u32>,
    have_numbers: Vec<u32>,
}

impl Card {
    pub fn score(&self) -> u32 {
        let num_matches = self.num_matches();
        if num_matches == 0 {
            return 0;
        }
        1 << (num_matches - 1)
    }

    pub fn num_matches(&self) -> usize {
        let winning_numbers = self.winning_numbers.iter().collect::<HashSet<_>>();
        self.have_numbers
            .iter()
            .filter(|num| winning_numbers.contains(num))
            .count()
    }
}

fn score_part2(cards: &[Card]) -> Option<usize> {
    let mut counts = vec![1; cards.len()];
    for (index, card) in cards.iter().enumerate() {
        let count = counts.get(index).copied()?;
        let num_matches = card.num_matches();
        for offset in 1..=num_matches {
            let next_index = index + offset;
            let next_count = counts.get_mut(next_index)?;
            *next_count += count
        }
    }
    Some(counts.iter().sum())
}

fn parse_card(line: &str) -> Option<Card> {
    card(line).ok().map(|(_, card)| card)
}

fn card(i: &str) -> IResult<Card> {
    let (i, _) = tuple((tag("Card"), character::space1))(i)?;
    let (i, id) = character::u32(i)?;
    let (i, _) = tuple((tag(":"), character::space1))(i)?;
    let (i, winning_numbers) = separated_list1(character::space1, character::u32)(i)?;
    let (i, _) = tuple((character::space1, tag("|"), character::space1))(i)?;
    let (i, have_numbers) = separated_list1(character::space1, character::u32)(i)?;
    let card = Card {
        id,
        winning_numbers,
        have_numbers,
    };
    Ok((i, card))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53";
        let expected = Card {
            id: 1,
            winning_numbers: vec![41, 48, 83, 86, 17],
            have_numbers: vec![83, 86, 6, 31, 17, 9, 48, 53],
        };
        let actual = parse_card(input);
        assert_eq!(Some(expected), actual);
    }

    #[test]
    fn test_examples_part1() {
        assert_eq!("13", Day04.part1(Example::Example, Debug::NotDebug));
    }

    #[test]
    fn test_real_part1() {
        assert_eq!("26443", Day04.part1(Example::Real, Debug::NotDebug));
    }

    #[test]
    fn test_examples_part2() {
        assert_eq!("30", Day04.part2(Example::Example, Debug::NotDebug));
    }

    #[test]
    fn test_real_part2() {
        assert_eq!("6284877", Day04.part2(Example::Real, Debug::NotDebug));
    }
}
