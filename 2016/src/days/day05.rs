use md5::Digest;

use crate::days::{Day, Debug, Example, Part};

pub struct Day05;

impl Day for Day05 {
    fn number(&self) -> u32 {
        5
    }

    fn run(&self, part: Part, example: Example, debug: Debug) {
        let answer = match part {
            Part::Part1 => self.part1(example, debug),
            Part::Part2 => self.part2(example, debug),
        };
        println!("{}", answer);
    }
}

impl Day05 {
    fn part1(&self, example: Example, _debug: Debug) -> String {
        find_password(&self.read_bytes(example))
    }

    fn part2(&self, _example: Example, _debug: Debug) -> String {
        todo!()
    }
}

const NUM_PASSWORD_DIGITS: usize = 8;
const LEADING_ZEROS_PREFIX: &str = "00000";
const NUM_LEADING_ZEROS: usize = LEADING_ZEROS_PREFIX.len();

fn find_password(door_id: &[u8]) -> String {
    (0..)
        .map(|index| create_input(door_id, index))
        .map(md5::compute)
        .filter_map(get_password_char)
        // .filter_map(|index| get_password_char_all(door_id, index))
        .take(NUM_PASSWORD_DIGITS)
        .collect()
}

fn get_password_char_all(door_id: &[u8], index: usize) -> Option<char> {
    let input = create_input(door_id, index);
    let digest = md5::compute(&input);
    let password_char = get_password_char(digest)?;
    println!(
        "***** Found password char! Index {} -> input {:02x?} -> digest {:?} -> char {}",
        index, input, digest, password_char
    );
    panic!();
    Some(password_char)
}

fn create_input(door_id: &[u8], index: usize) -> Vec<u8> {
    // if index % 10_000 == 0 {
    //     println!("Index {}", index);
    // }
    let mut input = door_id.to_vec();
    // if index == 479320 {
    //     println!("door_id: {:02x?}", door_id);
    // }
    input.extend(to_ascii_digits(index));
    // print!("Input for {:>4} is 0x", index);
    // for byte in input.iter() {
    //     print!("{:02x}", byte);
    // }
    // println!();
    input
}

fn get_password_char(digest: Digest) -> Option<char> {
    let digest_string = format!("{:x}", digest);
    // println!("  Digest: {}", digest_string);
    let rest = digest_string.strip_prefix(LEADING_ZEROS_PREFIX)?;
    // println!(
    //     "***** Found password char! {} *****",
    //     rest.chars().next().unwrap()
    // );
    Some(rest.chars().next().unwrap())
}

fn to_ascii_digits(index: usize) -> Vec<u8> {
    index.to_string().chars().map(|c| c as u8).collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    #[ignore]
    fn test_examples_part1() {
        assert_eq!("18f47a30", find_password(b"abc"));
    }

    #[test]
    #[ignore]
    fn test_real_part1() {
        assert_eq!("f77a0e6e", Day05.part1(Example::Real, Debug::NotDebug));
    }

    #[test]
    fn test_examples_part2() {}

    #[test]
    fn test_real_part2() {}
}
