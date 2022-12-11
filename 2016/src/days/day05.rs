use md5::Digest;
use std::fmt::Write;
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
        find_password(&self.read_file(example))
    }

    fn part2(&self, example: Example, _debug: Debug) -> String {
        find_password2(&self.read_file(example))
    }
}

const NUM_PASSWORD_DIGITS: usize = 8;
const LEADING_ZEROS_PREFIX: &str = "00000";
const NUM_LEADING_ZEROS: usize = LEADING_ZEROS_PREFIX.len();

fn find_password(door_id: &str) -> String {
    let mut password = String::new();
    let mut input = door_id.to_owned();
    for index in 0.. {
        input.truncate(door_id.len());
        write!(&mut input, "{}", index).unwrap();
        let digest = md5::compute(input.as_bytes()).0;
        let letter = match digest {
            [0, 0, x, ..] if x & 0xF0 == 0 => x & 0x0F,
            _ => continue,
        };
        write!(&mut password, "{:x}", letter).unwrap();
        if password.len() >= NUM_PASSWORD_DIGITS {
            break;
        }
    }
    password
}

fn find_password2(door_id: &str) -> String {
    let mut input = door_id.to_owned();
    let mut password_chars = vec![None; NUM_PASSWORD_DIGITS];
    for index in 0.. {
        input.truncate(door_id.len());
        write!(&mut input, "{}", index).unwrap();
        let digest = md5::compute(input.as_bytes()).0;
        let (position, letter) = match digest {
            [0, 0, x, y, ..] if x & 0xF0 == 0 => (x & 0x0F, (y & 0xF0) >> 4),
            _ => continue,
        };
        if let Some(c) = password_chars.get_mut(position as usize) {
            if c.is_none() {
                let letter = format!("{:x}", letter).chars().next().unwrap();
                *c = Some(letter);
            }
        }
        if password_chars.iter().all(|c| c.is_some()) {
            break;
        }
    }
    password_chars.into_iter().flatten().collect()
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
    // Some(password_char)
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

fn find_password_index_chars(door_id: &[u8]) -> impl Iterator<Item = (usize, char)> {
    let door_id = door_id.to_owned();
    (0..)
        .map(move |index| create_input(&door_id, index))
        .map(md5::compute)
        .filter_map(get_password_index_char)
}

fn get_password_index_char(digest: Digest) -> Option<(usize, char)> {
    let digest_string = format!("{:x}", digest);
    let mut rest = digest_string.strip_prefix(LEADING_ZEROS_PREFIX)?.chars();
    let position_char = rest.next().unwrap();
    if !matches!(position_char, '0'..='9') {
        return None;
    }
    let position = (position_char as u8 - b'0') as usize;
    if position >= NUM_PASSWORD_DIGITS {
        return None;
    }
    let char = rest.next().unwrap();
    Some((position, char))
}

fn to_ascii_digits(index: usize) -> Vec<u8> {
    index.to_string().chars().map(|c| c as u8).collect()
}

// fn to_ascii_digits_iter(index: usize) -> impl Iterator<Item = u8> {
//     let num_digits = if index == 0 {
//         1
//     } else {
//         (index as f64).log10() as u8
//     };
//     todo!()
// }

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    #[ignore]
    fn test_examples_part1() {
        assert_eq!("18f47a30", find_password("abc"));
    }

    #[test]
    #[ignore]
    fn test_real_part1() {
        assert_eq!("f77a0e6e", Day05.part1(Example::Real, Debug::NotDebug));
    }

    #[test]
    #[ignore]
    fn test_examples_part2() {
        assert_eq!("05ace8e3", find_password2("abc"));
    }

    #[test]
    #[ignore]
    fn test_real_part2() {
        assert_eq!("999828ec", Day05.part2(Example::Real, Debug::NotDebug));
    }
}
