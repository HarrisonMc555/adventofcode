use crate::days::{Day, Debug, Example, Part};
use std::iter::Peekable;

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

    fn part2(&self, _example: Example, _debug: Debug) -> usize {
        todo!()
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

        let chars_to_repeat = (0..length).map(|_| chars.next()).collect::<Option<Vec<_>>>()?;
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

// enum State {
//     Normal,
//     Length(usize),
//     Repeat(usize, usize),
//     FindingRepeat { length: usize, count: usize },
//     Repeating { length: usize, count: usize },
// }

trait PeekableExt<T: Iterator>: Sized {
    fn take_until<P>(self, predicate: P) -> TakeUntil<T, P>
    where
        P: FnMut(&T::Item) -> bool;
}

impl<T: Iterator> PeekableExt<T> for Peekable<T> {
    fn take_until<P>(self, predicate: P) -> TakeUntil<T, P>
    where
        P: FnMut(&T::Item) -> bool,
    {
        TakeUntil {
            inner: self,
            predicate,
        }
    }
}

struct TakeUntil<T: Iterator, P>
where
    P: FnMut(&T::Item) -> bool,
{
    inner: Peekable<T>,
    predicate: P,
}

impl<T: Iterator, P> Iterator for TakeUntil<T, P>
where
    P: FnMut(&T::Item) -> bool,
{
    type Item = T::Item;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(next) = self.inner.peek() {
            if (self.predicate)(next) {
                return self.inner.next(); 
            }
        }
        None
    }
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
    #[ignore]
    fn test_real_part2() {
        assert_eq!(0, Day09.part2(Example::Real, Debug::NotDebug));
    }
}
