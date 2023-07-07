use crate::days::{Day, Debug, Example, Part};
use itertools::Itertools;
use lazy_static::lazy_static;
use regex::{Captures, Regex};
use std::str::FromStr;

pub struct Day21;

const DEBUG: bool = true;

const PASSWORD_INITIAL: &str = "abcdefgh";
const PASSWORD_SCRAMBLED: &str = "fbgdceah";

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
    fn part1(&self, example: Example, _debug: Debug) -> String {
        let operations = Operation::read_operations(&self.read_file(example)).unwrap();
        let mut password = PASSWORD_INITIAL.bytes().collect::<Vec<_>>();
        Operation::execute_all(&operations, &mut password);
        String::from_utf8(password).unwrap()
    }

    fn part2(&self, example: Example, _debug: Debug) -> String {
        let operations = Operation::read_operations(&self.read_file(example)).unwrap();
        let mut password = PASSWORD_SCRAMBLED.bytes().collect::<Vec<_>>();
        Operation::execute_reverse_all(&operations, &mut password);
        let answer = String::from_utf8(password.clone()).unwrap();
        Operation::execute_all(&operations, &mut password);
        let round_trip = String::from_utf8(password).unwrap();
        if round_trip != PASSWORD_SCRAMBLED {
            panic!("Passwords do not match! Initial password: \"{}\", round trip: \"{}\" (unscrambled: \"{}\")", PASSWORD_SCRAMBLED, round_trip, answer);
        }
        answer
    }
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
enum Operation {
    SwapPosition(usize, usize),
    SwapLetter(u8, u8),
    RotateLeft(usize),
    RotateRight(usize),
    RotateFromChar(u8),
    ReversePositions(usize, usize),
    MovePosition(usize, usize),
}

impl Operation {
    pub fn execute(self, password: &mut [u8]) {
        match self {
            Operation::SwapPosition(x, y) => Self::swap_position(password, x, y),
            Operation::SwapLetter(x, y) => Self::swap_letter(password, x, y),
            Operation::RotateLeft(x) => Self::rotate_left(password, x),
            Operation::RotateRight(x) => Self::rotate_right(password, x),
            Operation::RotateFromChar(x) => Self::rotate_from_char(password, x),
            Operation::ReversePositions(x, y) => Self::reverse_positions(password, x, y),
            Operation::MovePosition(x, y) => Self::move_position(password, x, y),
        }
    }

    pub fn execute_reverse(self, password: &mut [u8]) {
        match self {
            Operation::SwapPosition(x, y) => Self::swap_position(password, x, y),
            Operation::SwapLetter(x, y) => Self::swap_letter(password, x, y),
            Operation::RotateLeft(x) => Self::rotate_right(password, x),
            Operation::RotateRight(x) => Self::rotate_left(password, x),
            Operation::RotateFromChar(x) => Self::rotate_from_char_reverse(password, x),
            Operation::ReversePositions(x, y) => Self::reverse_positions(password, x, y),
            Operation::MovePosition(x, y) => Self::move_position(password, y, x),
        }
    }

    pub fn execute_all(operations: &[Operation], password: &mut [u8]) {
        for operation in operations {
            debug_print!("{} -> ", String::from_utf8(password.to_vec()).unwrap());
            operation.execute(password);
            debug_println!(
                "{} ({:?})",
                String::from_utf8(password.to_vec()).unwrap(),
                operation
            );
        }
    }

    pub fn execute_reverse_all(operations: &[Operation], password: &mut [u8]) {
        for operation in operations.iter().rev() {
            debug_print!("{} -> ", String::from_utf8(password.to_vec()).unwrap());
            operation.execute_reverse(password);
            debug_println!(
                "{} ({:?})",
                String::from_utf8(password.to_vec()).unwrap(),
                operation
            );
        }
    }

    fn read_operations(string: &str) -> Result<Vec<Operation>, <Operation as FromStr>::Err> {
        string.lines().map(Operation::from_str).collect()
    }

    pub fn swap_position(password: &mut [u8], x: usize, y: usize) {
        password.swap(x, y)
    }

    pub fn swap_letter(password: &mut [u8], x: u8, y: u8) {
        let get_index = |c| Self::index_of(password, c);
        let index_x = get_index(x);
        let index_y = get_index(y);
        password.swap(index_x, index_y);
    }

    pub fn rotate_left(password: &mut [u8], x: usize) {
        password.rotate_left(x)
    }

    pub fn rotate_right(password: &mut [u8], x: usize) {
        password.rotate_right(x)
    }

    pub fn rotate_from_char(password: &mut [u8], x: u8) {
        let index = Self::index_of(password, x);
        let rotate_amount = 1 + index + if index >= 4 { 1 } else { 0 };
        password.rotate_right(rotate_amount % password.len())
    }

    pub fn rotate_from_char_reverse(password: &mut [u8], x: u8) {
        // forward (length 8):
        // 0 -> +1 -> 1  -> 1
        // 1 -> +2 -> 3  -> 3
        // 2 -> +3 -> 5  -> 5
        // 3 -> +4 -> 7  -> 7
        // 4 -> +6 -> 10 -> 2
        // 5 -> +7 -> 12 -> 4
        // 6 -> +8 -> 14 -> 6
        // 7 -> +9 -> 16 -> 0

        // forward (length 5):
        // 0 -> +1 -> 1  -> 1
        // 1 -> +2 -> 3  -> 3
        // 2 -> +3 -> 5  -> 0
        // 3 -> +4 -> 7  -> 2
        // 4 -> +6 -> 10 -> 0 (non-reversible!!!)

        if password.len() != 8 {
            panic!("Reversing \"rotate from char\" only supported for length 8");
        }

        let index = Self::index_of(password, x);
        let rotate_amount = if index == 0 {
            1
        } else if index % 2 == 0 {
            5 + (index / 2)
        } else {
            1 + (index / 2)
        };
        password.rotate_left(rotate_amount % password.len());
    }

    pub fn reverse_positions(password: &mut [u8], x: usize, y: usize) {
        password[x..=y].reverse();
    }

    pub fn move_position(password: &mut [u8], x: usize, y: usize) {
        if x < y {
            password[x..=y].rotate_left(1)
        } else {
            password[y..=x].rotate_right(1)
        }
    }

    fn index_of(password: &[u8], x: u8) -> usize {
        password.iter().position(|&c| c == x).unwrap()
    }
}

impl FromStr for Operation {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref SWAP_POSITION: Regex =
                Regex::new(r"^swap position (\d+) with position (\d+)$").unwrap();
            static ref SWAP_LETTER: Regex =
                Regex::new(r"^swap letter (\w) with letter (\w)$").unwrap();
            static ref ROTATE_LEFT: Regex = Regex::new(r"^rotate left (\d+) steps?$").unwrap();
            static ref ROTATE_RIGHT: Regex = Regex::new(r"^rotate right (\d+) steps?$").unwrap();
            static ref ROTATE_FROM_CHAR: Regex =
                Regex::new(r"^rotate based on position of letter (\w)$").unwrap();
            static ref REVERSE_POSITIONS: Regex =
                Regex::new(r"^reverse positions (\d+) through (\d+)").unwrap();
            static ref MOVE_POSITION: Regex =
                Regex::new(r"^move position (\d+) to position (\d+)$").unwrap();
        }

        let get_usize = |caps: &Captures, index| caps.get(index).unwrap().as_str().parse().unwrap();
        let get_char = |caps: &Captures, index| {
            caps.get(index)
                .unwrap()
                .as_str()
                .bytes()
                .exactly_one()
                .unwrap()
        };
        if let Some(caps) = SWAP_POSITION.captures(s) {
            let x = get_usize(&caps, 1);
            let y = get_usize(&caps, 2);
            Ok(Self::SwapPosition(x, y))
        } else if let Some(caps) = SWAP_LETTER.captures(s) {
            let x = get_char(&caps, 1);
            let y = get_char(&caps, 2);
            Ok(Self::SwapLetter(x, y))
        } else if let Some(caps) = ROTATE_LEFT.captures(s) {
            let x = get_usize(&caps, 1);
            Ok(Self::RotateLeft(x))
        } else if let Some(caps) = ROTATE_RIGHT.captures(s) {
            let x = get_usize(&caps, 1);
            Ok(Self::RotateRight(x))
        } else if let Some(caps) = ROTATE_FROM_CHAR.captures(s) {
            let x = get_char(&caps, 1);
            Ok(Self::RotateFromChar(x))
        } else if let Some(caps) = REVERSE_POSITIONS.captures(s) {
            let x = get_usize(&caps, 1);
            let y = get_usize(&caps, 2);
            Ok(Self::ReversePositions(x, y))
        } else if let Some(caps) = MOVE_POSITION.captures(s) {
            let x = get_usize(&caps, 1);
            let y = get_usize(&caps, 2);
            Ok(Self::MovePosition(x, y))
        } else {
            Err(format!("Invalid operation \"{}\"", s))
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_examples_part1() {
        let password = &mut "abcde".bytes().collect::<Vec<_>>();
        op("swap position 4 with position 0").execute(password);
        assert_eq!("ebcda", str(password));
        op("swap letter d with letter b").execute(password);
        assert_eq!("edcba", str(password));
        op("reverse positions 0 through 4").execute(password);
        assert_eq!("abcde", str(password));
        op("rotate left 1 step").execute(password);
        assert_eq!("bcdea", str(password));
        op("move position 1 to position 4").execute(password);
        assert_eq!("bdeac", str(password));
        op("move position 3 to position 0").execute(password);
        assert_eq!("abdec", str(password));
        op("rotate based on position of letter b").execute(password);
        assert_eq!("ecabd", str(password));
        op("rotate based on position of letter d").execute(password);
        assert_eq!("decab", str(password));
    }

    #[test]
    fn test_real_part1() {
        assert_eq!("dgfaehcb", Day21.part1(Example::Real, Debug::NotDebug));
    }

    #[test]
    fn test_examples_part2() {
        // Not supported for length 5
        // let password = &mut "decab".bytes().collect::<Vec<_>>();
        // op("rotate based on position of letter d").execute_reverse(password);
        // assert_eq!("ecabd", str(password));
        // op("rotate based on position of letter b").execute_reverse(password);
        // assert_eq!("abdec", str(password));
        let password = &mut "abdec".bytes().collect::<Vec<_>>();
        op("move position 3 to position 0").execute_reverse(password);
        assert_eq!("bdeac", str(password));
        op("move position 1 to position 4").execute_reverse(password);
        assert_eq!("bcdea", str(password));
        op("rotate left 1 step").execute_reverse(password);
        assert_eq!("abcde", str(password));
        op("reverse positions 0 through 4").execute_reverse(password);
        assert_eq!("edcba", str(password));
        op("swap letter d with letter b").execute_reverse(password);
        assert_eq!("ebcda", str(password));
        op("swap position 4 with position 0").execute_reverse(password);
        assert_eq!("abcde", str(password));
    }

    #[test]
    fn test_real_part2() {
        assert_eq!("fdhgacbe", Day21.part2(Example::Real, Debug::NotDebug));
    }

    #[test]
    fn test_round_trip() {
        let operations = [
            op("swap position 4 with position 0"),
            op("swap letter d with letter b"),
            op("reverse positions 0 through 4"),
            op("rotate left 1 step"),
            op("move position 1 to position 4"),
            op("move position 3 to position 0"),
            op("rotate based on position of letter b"),
            op("rotate based on position of letter d"),
        ];
        assert_round_trip("abcdefgh", &operations);

        let operations = [
            op("rotate based on position of letter d"),
            op("rotate based on position of letter b"),
            op("move position 3 to position 0"),
            op("move position 1 to position 4"),
            op("rotate left 1 step"),
            op("reverse positions 0 through 4"),
            op("swap letter d with letter b"),
            op("swap position 4 with position 0"),
        ];
        assert_round_trip("abcdefgh", &operations);
    }

    fn op(s: &str) -> Operation {
        s.parse().unwrap()
    }

    fn str(bytes: &[u8]) -> String {
        String::from_utf8(bytes.to_vec()).unwrap()
    }

    fn assert_round_trip(password: &str, operations: &[Operation]) {
        let before = password;
        debug_println!("* Round trip for {}", password);
        let mut password = password.bytes().collect::<Vec<_>>();

        let mut forward = Vec::new();
        for operation in operations {
            forward.push(str(&password));
            debug_print!("{} -> ", String::from_utf8(password.to_vec()).unwrap());
            operation.execute(&mut password);
            debug_println!(
                "{} ({:?})",
                String::from_utf8(password.to_vec()).unwrap(),
                operation
            );
        }

        debug_println!("* Finished forward, beginning backward");
        for (operation, expected) in operations.iter().zip(forward.iter()).rev() {
            let before = str(&password);
            debug_print!("{} -> ", String::from_utf8(password.to_vec()).unwrap());
            operation.execute_reverse(&mut password);
            debug_println!(
                "{} ({:?})",
                String::from_utf8(password.to_vec()).unwrap(),
                operation
            );
            let actual = str(&password);
            assert_eq!(
                expected, &actual,
                "Reversing operation {:?} transformed \"{}\" into \"{}\", expected \"{}\"",
                operation, before, actual, expected
            );
        }

        let after = String::from_utf8(password).unwrap();
        assert_eq!(before, after);
        debug_println!();
    }
}
