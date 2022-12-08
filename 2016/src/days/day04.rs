use counter::Counter;
use lazy_static::lazy_static;
use regex::Regex;

use crate::days::{Day, Debug, Example, Part};

pub struct Day04;

impl Day for Day04 {
    fn number(&self) -> u32 {
        4
    }

    fn run(&self, part: Part, example: Example, debug: Debug) {
        let answer = match part {
            Part::Part1 => self.part1(example, debug),
            Part::Part2 => self.part2(example, debug),
        };
        println!("{}", answer);
    }
}

impl Day04 {
    fn part1(&self, example: Example, _debug: Debug) -> usize {
        get_answer(&self.read_file(example)).unwrap()
    }

    fn part2(&self, _example: Example, _debug: Debug) -> usize {
        unimplemented!()
    }
}

const CHECKSUM_LEN: usize = 5;

fn get_answer(text: &str) -> Option<usize> {
    let rooms = parse_rooms(text)?;
    Some(
        rooms
            .into_iter()
            .filter(Room::is_real)
            .map(|room| room.sector_id)
            .sum(),
    )
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
struct Room {
    letters: Vec<char>,
    sector_id: usize,
    checksum: Vec<char>,
}

fn parse_rooms(text: &str) -> Option<Vec<Room>> {
    text.trim().lines().map(Room::parse).collect()
}

impl Room {
    fn is_real(&self) -> bool {
        // println!("Is this room real? {:?}", self);
        let mut counter = self.letters.iter().collect::<Counter<_>>();
        // println!("Counter: {:?}", counter);
        let mut expected_checksum = Vec::new();
        while expected_checksum.len() < CHECKSUM_LEN {
            let most_common = counter.most_common_tiebreaker(|a, b| a.cmp(&b));
            // println!("\tMost common letters: {:?}", most_common);
            for (letter, _) in most_common {
                // println!("\t\tAdding {} to expected checksum", letter);
                expected_checksum.push(*letter);
                if expected_checksum.len() >= CHECKSUM_LEN {
                    break;
                }
                counter.remove(letter);
            }
            // println!("Now, counter: {:?}", counter);
        }
        self.checksum == expected_checksum
    }

    fn parse(line: &str) -> Option<Self> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^([a-z-]+)-(\d+)\[([a-z]+)\]$").unwrap();
        }
        let caps = RE.captures(line)?;
        let letters = caps
            .get(1)
            .unwrap()
            .as_str()
            .chars()
            .filter(char::is_ascii_alphabetic)
            .collect();
        let sector_id = caps.get(2).unwrap().as_str().parse().ok()?;
        let checksum = to_chars(caps.get(3).unwrap().as_str());
        Some(Room {
            letters,
            sector_id,
            checksum,
        })
    }
}

fn to_chars<T: AsRef<str>>(string: T) -> Vec<char> {
    string.as_ref().chars().collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_examples_part1() {
        assert!(is_real("aaaaa-bbb-z-y-x-123[abxyz]"));
        assert!(is_real("a-b-c-d-e-f-g-h-987[abcde]"));
        assert!(is_real("not-a-real-room-404[oarel]"));
        assert!(!is_real("totally-real-room-200[decoy]"));
        let answer = get_answer(include_str!("../../static/example04.txt"));
        assert_eq!(Some(1514), answer);
    }

    #[test]
    fn test_real_part1() {
        assert_eq!(278221, Day04.part1(Example::Real, Debug::NotDebug));
    }

    #[test]
    fn test_examples_part2() {}

    fn is_real(line: &str) -> bool {
        let room = Room::parse(line).unwrap();
        room.is_real()
    }
}
