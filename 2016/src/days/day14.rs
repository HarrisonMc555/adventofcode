use std::collections::VecDeque;

use crate::days::{Day, Debug, Example, Part};

const DEBUG: bool = false;

const KEY_CHAR_REPEAT_LEN_1: usize = 3;
const KEY_CHAR_REPEAT_LEN_2: usize = 5;
const KEY_INDEX: usize = 64;
const PART1_STRETCH_NUM: usize = 0;
const PART2_STRETCH_NUM: usize = 2016;

pub struct Day14;

impl Day for Day14 {
    fn number(&self) -> u32 {
        14
    }

    fn run(&self, part: Part, example: Example, debug: Debug) {
        let answer = match part {
            Part::Part1 => self.part1(example, debug),
            Part::Part2 => self.part2(example, debug),
        };
        println!("{}", answer);
    }
}

impl Day14 {
    fn part1(&self, example: Example, _debug: Debug) -> usize {
        let salt = self.read_file(example);
        let key_generator = KeyGenerator::new(salt, PART1_STRETCH_NUM);
        let keys = key_generator.create_keys(KEY_INDEX);
        keys[KEY_INDEX - 1]
    }

    fn part2(&self, example: Example, _debug: Debug) -> usize {
        let salt = self.read_file(example);
        let key_generator = KeyGenerator::new(salt, PART2_STRETCH_NUM);
        let keys = key_generator.create_keys(KEY_INDEX);
        keys[KEY_INDEX - 1]
    }
}

#[derive(Clone)]
struct KeyGenerator {
    salt: String,
    // salt: Vec<u8>,
    possible_keys: VecDeque<PossibleKey>,
    keys: Vec<usize>,
    index: usize,
    stretch_num: usize,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
enum PossibleKey {
    Possible(usize, u8),
    Key(usize),
}

impl KeyGenerator {
    pub fn new(salt: String, stretch_num: usize) -> Self {
        Self {
            salt,
            possible_keys: VecDeque::new(),
            keys: Vec::new(),
            index: 0,
            stretch_num,
        }
    }

    pub fn create_keys(mut self, num_keys: usize) -> Vec<usize> {
        while self.keys.len() < num_keys {
            self.step();
        }
        self.keys
    }

    pub fn step(&mut self) {
        let hash = self.compute_hash();

        for possible_key in self.possible_keys.iter_mut() {
            let (index, c) = match possible_key {
                PossibleKey::Possible(index, c) => (*index, *c),
                PossibleKey::Key(_) => continue,
            };
            if contains_repeated_match(&hash, KEY_CHAR_REPEAT_LEN_2, &c) {
                *possible_key = PossibleKey::Key(index);
            }
        }
        loop {
            let Some(possible_key) = self.possible_keys.get(0).copied() else {
                break;
            };
            let index = match possible_key {
                PossibleKey::Possible(index, _) => index,
                PossibleKey::Key(index) => {
                    self.keys.push(index);
                    self.possible_keys.pop_front();
                    continue;
                }
            };
            if self.index > index + 1000 {
                self.possible_keys.pop_front();
            }
            break;
        }

        if let Some(c) = get_repeated_element(&hash, KEY_CHAR_REPEAT_LEN_1) {
            self.possible_keys
                .push_back(PossibleKey::Possible(self.index, c));
        };

        self.index += 1;
    }

    fn compute_hash(&self) -> String {
        let mut hash = format!("{}{}", self.salt, self.index);
        for _ in 0..=self.stretch_num {
            hash = format!("{:x}", md5::compute(hash));
        }
        hash
    }
}

fn get_repeated_element<T: PartialEq + Clone, S: AsRef<[T]>>(slice: S, len: usize) -> Option<T> {
    let slice = slice.as_ref();
    let max_index = slice.len().checked_sub(len)?;
    'outer: for index in 0..=max_index {
        let first = slice.get(index)?;
        for offset in 1..len {
            let next_index = index + offset;
            let next = slice.get(next_index)?;
            if first != next {
                continue 'outer;
            }
        }
        return Some(first.clone());
    }
    None
}

fn contains_repeated_match<T: PartialEq, S: AsRef<[T]>>(slice: S, len: usize, value: &T) -> bool {
    if len == 0 {
        return true;
    }
    let slice = slice.as_ref();
    let mut cur_len = 0;
    if slice.len() < len {
        return false;
    }
    for (index, element) in slice.iter().enumerate() {
        if element == value {
            cur_len += 1;
            if cur_len >= len {
                return true;
            }
        } else {
            if index + len > slice.len() {
                return false;
            }
            cur_len = 0;
        }
    }
    false
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_get_repeated_element() {
        let empty: &[u8] = &[];
        assert_eq!(None, get_repeated_element(empty, 2));
        assert_eq!(None, get_repeated_element(empty, 1));
        assert_eq!(None, get_repeated_element(empty, 0));
        assert_eq!(Some(b'a'), get_repeated_element("a", 1));
        assert_eq!(Some(b'a'), get_repeated_element("ab", 1));
        assert_eq!(Some(b'a'), get_repeated_element("abb", 1));
        assert_eq!(Some(b'b'), get_repeated_element("abb", 2));
        assert_eq!(Some(b'b'), get_repeated_element("abb", 2));
        assert_eq!(Some(b'b'), get_repeated_element("abbccccc", 2));
        assert_eq!(Some(b'c'), get_repeated_element("abbccccc", 3));
        assert_eq!(Some(b'c'), get_repeated_element("abbccccc", 4));
        assert_eq!(Some(b'c'), get_repeated_element("abbccccc", 5));
    }

    #[test]
    fn test_contains_repeated_match() {
        assert!(contains_repeated_match("", 0, &b'a'));
        assert!(!contains_repeated_match("", 1, &b'a'));
        assert!(contains_repeated_match("a", 0, &b'a'));
        assert!(contains_repeated_match("a", 1, &b'a'));
        assert!(!contains_repeated_match("a", 2, &b'a'));
        assert!(!contains_repeated_match("bbb", 1, &b'a'));
        assert!(!contains_repeated_match("bbb", 2, &b'a'));
        assert!(contains_repeated_match("bbb", 1, &b'b'));
        assert!(contains_repeated_match("bbb", 2, &b'b'));
        assert!(contains_repeated_match("bbb", 3, &b'b'));
        assert!(!contains_repeated_match("bbb", 4, &b'b'));
        assert!(contains_repeated_match("ababa", 1, &b'a'));
        assert!(contains_repeated_match("ababa", 1, &b'b'));
        assert!(!contains_repeated_match("ababa", 2, &b'a'));
        assert!(!contains_repeated_match("ababa", 2, &b'b'));
        assert!(!contains_repeated_match("abba", 2, &b'a'));
        assert!(contains_repeated_match("abba", 2, &b'b'));
    }

    #[test]
    fn test_example_part1() {
        let salt = "abc";
        let key_generator = KeyGenerator::new(salt.to_owned(), PART1_STRETCH_NUM);
        let keys = key_generator.create_keys(2);
        assert_eq!(&[39, 92], &keys[..2]);
        let key_generator = KeyGenerator::new(salt.to_owned(), PART1_STRETCH_NUM);
        let keys = key_generator.create_keys(64);
        assert_eq!(22728, keys[64 - 1]);
    }

    #[test]
    fn test_example_part2() {
        let salt = "abc";
        let key_generator = KeyGenerator::new(salt.to_owned(), PART2_STRETCH_NUM);
        let keys = key_generator.create_keys(1);
        assert_eq!(10, keys[0]);
        let key_generator = KeyGenerator::new(salt.to_owned(), PART2_STRETCH_NUM);
        let keys = key_generator.create_keys(64);
        assert_eq!(22551, keys[64 - 1]);
    }
}
