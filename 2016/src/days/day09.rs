use crate::days::{Day, Debug, Example, Part};

pub struct Day09;

impl Day for Day09 {
    fn number(&self) -> u32 {
        9
    }

    fn run(&self, part: Part, example: Example, debug: Debug) {
        let answer = match part {
            Part::Part1 => self.part1(example, debug),
            Part::Part2 => self.part2(example, debug),
        };
        println!("{}", answer);
    }
}

impl Day09 {
    fn part1(&self, example: Example, _debug: Debug) -> usize {
        decompress(self.read_file(example).trim()).unwrap().len()
    }

    fn part2(&self, example: Example, _debug: Debug) -> usize {
        decompressed_len(self.read_file(example).trim()).unwrap()
    }
}

const BASE: u32 = 10;
fn decompress(input: &str) -> Option<String> {
    let mut output = String::new();

    let mut chars = input.chars();
    'outer: loop {
        'normal: loop {
            let Some(c) = chars.next() else {
                break 'outer;
            };
            if c == '(' {
                break 'normal;
            }
            output.push(c);
        }

        let mut length = 0;
        let mut found_length_char = false;
        'length: loop {
            let c = chars.next()?;
            if c == 'x' {
                if !found_length_char {
                    return None;
                }
                break 'length;
            }
            let digit = c.to_digit(BASE)? as usize;
            length = length * BASE as usize + digit;
            found_length_char = true;
        }

        let mut count = 0;
        let mut found_count_char = false;
        'count: loop {
            let c = chars.next()?;
            if c == ')' {
                if !found_count_char {
                    return None;
                }
                break 'count;
            }
            let digit = c.to_digit(BASE)? as usize;
            count = count * BASE as usize + digit;
            found_count_char = true;
        }

        let chars_to_repeat = (0..length)
            .map(|_| chars.next())
            .collect::<Option<Vec<_>>>()?;
        for _ in 0..count {
            for c in chars_to_repeat.iter() {
                output.push(*c);
            }
        }
    }
    Some(output)

    //
    //
    // let mut state = State::Normal;
    // fn append_digit(mut num: usize, digit: char) -> Option<usize> {
    //     let digit = digit.to_digit(BASE)? as usize;
    //     Some(num * BASE as usize + digit)
    // }
    // for c in input.chars() {
    //     match &state {
    //         State::Normal if c == '(' => state = State::Length(0),
    //         State::Normal => output.push(c),
    //         State::Length(length) if c == 'x' => state = State::Repeat(*length, 0),
    //         State::Length(length) => state = State::Length(append_digit(*length, c)?),
    //         State::Repeat(length, count) if c == ')' => {
    //             state = State::Repeating {
    //                 length: *length,
    //                 count: *count,
    //             }
    //         }
    //         State::Repeat(length, count) => {
    //             state = State::Repeat(*length, append_digit(*count, c)?)
    //         }
    //         State::Repeating { length, count: 0 } => state = State::Normal,
    //         State::Repeating { length, count } => output.extend(),
    //     }
    // }
    // Some(output)
}

fn decompressed_len(input: &str) -> Option<usize> {
    let mut len = 0;

    let mut chars = input.chars();
    'outer: loop {
        'normal: loop {
            let Some(c) = chars.next() else {
                break 'outer;
            };
            if c == '(' {
                break 'normal;
            }
            len += 1;
        }

        let mut length = 0;
        let mut found_length_char = false;
        'length: loop {
            let c = chars.next()?;
            if c == 'x' {
                if !found_length_char {
                    return None;
                }
                break 'length;
            }
            let digit = c.to_digit(BASE)? as usize;
            length = length * BASE as usize + digit;
            found_length_char = true;
        }

        let mut count = 0;
        let mut found_count_char = false;
        'count: loop {
            let c = chars.next()?;
            if c == ')' {
                if !found_count_char {
                    return None;
                }
                break 'count;
            }
            let digit = c.to_digit(BASE)? as usize;
            count = count * BASE as usize + digit;
            found_count_char = true;
        }

        let string_to_repeat = (0..length)
            .map(|_| chars.next())
            .collect::<Option<String>>()?;
        let decompressed_length = decompressed_len(&string_to_repeat)?;
        len += count * decompressed_length;
    }
    Some(len)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_examples_part1() {
        assert_eq!("ADVENT", decompress("ADVENT").unwrap());
        assert_eq!("ABBBBBC", decompress("A(1x5)BC").unwrap());
        assert_eq!("ABCBCDEFEFG", decompress("A(2x2)BCD(2x2)EFG").unwrap());
        assert_eq!("(1x3)A", decompress("(6x1)(1x3)A").unwrap());
        assert_eq!("X(3x3)ABC(3x3)ABCY", decompress("X(8x2)(3x3)ABCY").unwrap());
    }

    #[test]
    fn test_real_part1() {
        assert_eq!(183269, Day09.part1(Example::Real, Debug::NotDebug));
    }

    #[test]
    fn test_examples_part2() {
        assert_eq!(
            "XABCABCABCABCABCABCY".len(),
            decompressed_len("X(8x2)(3x3)ABCY").unwrap()
        );
        assert_eq!(
            241920,
            decompressed_len("(27x12)(20x12)(13x14)(7x10)(1x12)A").unwrap()
        );
        assert_eq!(
            445,
            decompressed_len("(25x3)(3x3)ABC(2x3)XY(5x2)PQRSTX(18x9)(3x2)TWO(5x7)SEVEN").unwrap()
        );
    }

    #[test]
    fn test_real_part2() {
        assert_eq!(11317278863, Day09.part2(Example::Real, Debug::NotDebug));
    }
}
