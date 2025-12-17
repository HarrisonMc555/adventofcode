use std::{num::ParseIntError, str::FromStr};

use crate::days::{Day, Debug, Example, Part};

pub struct Day05;

impl Day for Day05 {
    fn number(&self) -> u32 {
        5
    }

    fn part1(&self, example: Example, _debug: Debug) -> String {
        let text = self.read_file(Part::Part1, example);
        let (ranges, ingredients) = parse_text(&text).unwrap();
        // println!("Before:\n");
        // for range in ranges.iter() {
        //     println!("{range}");
        // }
        let ranges = merge_ranges(ranges);
        // println!();
        // println!("After:\n");
        // for range in ranges.iter() {
        //     println!("{range}");
        // }
        let num_fresh_ingredients = ingredients
            .into_iter()
            .filter(|ingredient| is_fresh(&ranges, *ingredient))
            .count();
        num_fresh_ingredients.to_string()
    }

    fn part2(&self, example: Example, _debug: Debug) -> String {
        let text = self.read_file(Part::Part2, example);
        let (ranges, _ingredients) = parse_text(&text).unwrap();
        let ranges = merge_ranges(ranges);
        let num_fresh_ingredients = ranges.into_iter().map(IngredientRange::size).sum::<u64>();
        num_fresh_ingredients.to_string()
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct IngredientID(u64);

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct IngredientRange {
    from: IngredientID,
    to: IngredientID,
}

fn merge_ranges(ranges: Vec<IngredientRange>) -> Vec<IngredientRange> {
    let mut merged_ranges = Vec::new();
    for range in ranges {
        insert_into_merged_ranges(&mut merged_ranges, range);
    }
    merged_ranges
}

fn insert_into_merged_ranges(merged_ranges: &mut Vec<IngredientRange>, mut range: IngredientRange) {
    // println!(
    //     "Inserting {range} into [{}]",
    //     merged_ranges.iter().map(|r| r.to_string()).join(", ")
    // );
    let mut index = match merged_ranges.binary_search(&range) {
        Ok(index) => index,
        Err(index) => index,
    };
    // println!("\tRange matches index {index}");

    if let Some(prev_index) = index.checked_sub(1) {
        if let Some(prev_range) = merged_ranges.get(prev_index) {
            if range.from.0 <= prev_range.to.0 + 1 {
                // println!("\tMerging previous range {prev_range} into {range}");
                range.from = range.from.min(prev_range.from);
                range.to = range.to.max(prev_range.to);
                // println!("\t\tAfter merging previous, range is {range}");
                merged_ranges.remove(prev_index);
                index = prev_index;
            }
        }
    }
    let index = index;

    let mut num_to_merge = 0;
    for next_index in index..merged_ranges.len() {
        // println!("\t\tindex = {next_index}");
        let Some(next_range) = merged_ranges.get(next_index) else {
            // println!("\t\tIndex {next_index} out of range");
            break;
        };
        if next_range.from.0 > range.to.0 + 1 {
            // println!("\t\tNext range {next_range} does NOT overlap with range {range}");
            break;
        }
        // println!("\t\tMerging {next_range} into {range}");
        num_to_merge += 1;
        range.to = range.to.max(next_range.to);
        // println!("\t\t\tAfter merging, range is {range}");
    }
    merged_ranges.drain(index..(index + num_to_merge));
    merged_ranges.insert(index, range)
}

fn is_fresh(ranges: &[IngredientRange], ingredient: IngredientID) -> bool {
    // println!("Is {ingredient} fresh?");
    let index = match ranges.binary_search_by_key(&ingredient, |range| range.from) {
        Ok(index) => index,
        Err(index) => index,
    };
    if let Some(range) = ranges.get(index) {
        if range.from <= ingredient && ingredient <= range.to {
            // println!("\t{ingredient} is fresh from next range {range}");
            return true;
        }
    };
    if let Some(prev_index) = index.checked_sub(1) {
        if let Some(prev_range) = ranges.get(prev_index) {
            if prev_range.from <= ingredient && ingredient <= prev_range.to {
                // println!("\t{ingredient} is fresh from previous range {prev_range}");
                return true;
            }
        };
    }
    // println!("\t{ingredient} is NOT fresh");
    false
}

impl IngredientRange {
    pub fn size(self) -> u64 {
        self.to.0 - self.from.0 + 1
    }
}

fn parse_text(
    text: &str,
) -> Result<(Vec<IngredientRange>, Vec<IngredientID>), ParseIngredientRangeError> {
    let (ranges_text, ingredients_text) = text.split_once("\n\n").unwrap();
    let ranges = ranges_text
        .lines()
        .map(|line| line.parse::<IngredientRange>())
        .collect::<Result<Vec<_>, _>>()?;
    let ingredients = ingredients_text
        .lines()
        .map(|line| line.parse::<IngredientID>())
        .collect::<Result<Vec<_>, _>>()?;
    Ok((ranges, ingredients))
}

impl FromStr for IngredientID {
    type Err = ParseIngredientRangeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(
            s.parse().map_err(ParseIngredientRangeError::InvalidID)?,
        ))
    }
}

impl FromStr for IngredientRange {
    type Err = ParseIngredientRangeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let pieces = s.split("-").collect::<Vec<_>>();
        let [from, to] = &pieces[..] else {
            return Err(ParseIngredientRangeError::InvalidFormat);
        };
        let from = from.parse()?;
        let to = to.parse()?;
        Ok(Self { from, to })
    }
}

#[derive(Debug, PartialEq, Eq)]
enum ParseIngredientRangeError {
    InvalidID(ParseIntError),
    InvalidFormat,
}

impl std::fmt::Display for ParseIngredientRangeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseIngredientRangeError::InvalidID(e) => write!(f, "Invalid ID: {e}")?,
            ParseIngredientRangeError::InvalidFormat => write!(f, "Invalid format")?,
        }
        Ok(())
    }
}

impl std::error::Error for ParseIngredientRangeError {}

impl std::fmt::Display for IngredientRange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}-{}", self.from, self.to)
    }
}

impl std::fmt::Display for IngredientID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_examples_part1() {
        assert_eq!("3", Day05.part1(Example::Example, Debug::NotDebug));
    }

    #[test]
    fn test_real_part1() {
        assert_eq!("607", Day05.part1(Example::Real, Debug::NotDebug));
    }

    #[test]
    fn test_examples_part2() {
        assert_eq!("14", Day05.part2(Example::Example, Debug::NotDebug));
    }

    #[test]
    fn test_real_part2() {
        assert_eq!("342433357244012", Day05.part2(Example::Real, Debug::NotDebug));
    }
}
