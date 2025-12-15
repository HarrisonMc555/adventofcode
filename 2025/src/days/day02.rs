use itertools::Itertools;

use crate::days::{Day, Debug, Example, Part};

pub struct Day02;

impl Day for Day02 {
    fn number(&self) -> u32 {
        2
    }

    fn part1(&self, example: Example, _debug: Debug) -> String {
        let text = self.read_file(Part::Part1, example);
        let ranges = parse_ranges(&text).unwrap();
        let ranges = ranges.into_iter().map(RangeDigits::<10>::from_range);
        ranges
            .into_iter()
            .flat_map(get_invalid_ids)
            .sum::<u64>()
            .to_string()
    }

    fn part2(&self, example: Example, _debug: Debug) -> String {
        let text = self.read_file(Part::Part1, example);
        let ranges = parse_ranges(&text).unwrap();
        let ranges = ranges
            .into_iter()
            .map(RangeDigits::<10>::from_range)
            .collect::<Vec<_>>();
        ranges
            .iter()
            .flat_map(get_invalid_ids_part2)
            .sum::<u64>()
            .to_string()
    }
}

fn parse_ranges<const BASE: u32>(text: &str) -> Result<Vec<Range<BASE>>, String> {
    text.trim_ascii_end()
        .split(",")
        .map(|range| range.parse())
        .collect()
}

fn get_invalid_ids<const BASE: u32>(range: RangeDigits<BASE>) -> impl Iterator<Item = u64> {
    let RangeDigits { from, to } = range;
    let min_value = from.to_value();
    let max_value = to.to_value();
    let min_num_digits = from.digits.len().max(1);
    let max_num_digits = to.digits.len().max(1);
    let num_digits_iter = min_num_digits..=max_num_digits;
    let part_len_iter = num_digits_iter.filter_map(|num_digits| {
        if num_digits % 2 == 0 {
            Some(num_digits / 2)
        } else {
            None
        }
    });
    let part_start_digits_iter = part_len_iter.map(move |part_len| {
        if part_len * 2 == min_num_digits {
            let mut min_digits = Vec::with_capacity(part_len);
            min_digits.extend(&from.digits[part_len..]);
            // println!("\tUsing min value digits {min_digits:?}");
            Digits { digits: min_digits } as Digits<BASE>
        } else {
            // println!("Using minimum value with {num_digits} digits");
            Digits::min_value(part_len)
        }
    });
    let invalid_id_part_digits_iter =
        part_start_digits_iter.flat_map(move |start_digits| DigitsIterator::new(start_digits));
    let invalid_id_digits = invalid_id_part_digits_iter.map(|digits| {
        // println!("\tdouble digits: {digits:?}");
        let mut all_digits = Vec::with_capacity(digits.digits.len() * 2);
        all_digits.extend(&digits.digits);
        all_digits.extend(&digits.digits);
        Digits::<BASE> { digits: all_digits }
    });
    let invalid_ids = invalid_id_digits.map(|digits| digits.to_value());
    invalid_ids
        .skip_while(move |value| *value < min_value)
        .take_while(move |value| *value <= max_value)
}

fn get_invalid_ids_part2<const BASE: u32>(range: &RangeDigits<BASE>) -> impl Iterator<Item = u64> {
    let RangeDigits { from, to } = range;
    let min_value = from.to_value();
    let max_value = to.to_value();
    let min_num_digits = from.digits.len().max(1);
    let max_num_digits = to.digits.len().max(1);
    let num_digits_iter = min_num_digits..=max_num_digits;
    let part_len_and_num_parts_iter = num_digits_iter.flat_map(part_len_and_num_parts_iter);
    let invalid_ids = part_len_and_num_parts_iter
        .flat_map(move |(part_len, num_parts)| {
            get_invalid_ids_with_parts(&range, part_len, num_parts)
        })
        .unique()
        .sorted();
    invalid_ids
        .skip_while(move |value| *value < min_value)
        .take_while(move |value| *value <= max_value)
}

fn get_invalid_ids_with_parts<const BASE: u32>(
    range: &RangeDigits<BASE>,
    part_len: usize,
    num_parts: usize,
) -> impl Iterator<Item = u64> {
    let num_digits = part_len * num_parts;
    let start_digits = if num_digits == range.from.digits.len() {
        let mut min_digits = Vec::with_capacity(part_len);
        let start_index = range.from.digits.len() - part_len;
        min_digits.extend(&range.from.digits[start_index..]);
        // println!("\tUsing min value digits {min_digits:?}");
        Digits { digits: min_digits } as Digits<BASE>
    } else {
        Digits::min_value(part_len)
    };
    let invalid_id_part_digits_iter = DigitsIterator::new(start_digits);
    let invalid_id_digits = invalid_id_part_digits_iter.map(move |digits| {
        let all_digits = (0..num_parts)
            .flat_map(|_| &digits.digits)
            .copied()
            .collect::<Vec<_>>();
        Digits::<BASE> { digits: all_digits }
    });
    // invalid_id_digits.map(|digits| digits.to_value())
    let invalid_ids = invalid_id_digits
        .map(|digits| digits.to_value())
        .collect::<Vec<_>>();
    // println!("Invalid IDs for range {range:?} with part length {part_len} and {num_parts} parts:");
    // for invalid_id in invalid_ids.iter() {
    //     println!("\tInvalid ID: {invalid_id}");
    // }
    invalid_ids.into_iter()
}

fn part_len_and_num_parts_iter(num_digits: usize) -> impl Iterator<Item = (usize, usize)> {
    (1..num_digits).filter_map(move |part_len| {
        if num_digits % part_len == 0 {
            Some((part_len, num_digits / part_len))
        } else {
            None
        }
    })
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
struct Range<const BASE: u32> {
    from: u64,
    to: u64,
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
struct RangeDigits<const BASE: u32> {
    from: Digits<BASE>,
    to: Digits<BASE>,
}

/// Digits are listed from least-significant to most-signifcant. This means that if you print the contents of the
/// `digits` vector, they will be printed backwards.
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
struct Digits<const BASE: u32> {
    digits: Vec<u8>,
}

impl<const BASE: u32> Digits<BASE> {
    pub fn from_value(value: u64) -> Self {
        let mut value = value;
        let mut digits = Vec::new();
        while value > 0 {
            digits.push((value % BASE as u64) as u8);
            value /= BASE as u64;
        }
        Self { digits }
    }

    pub fn to_value(&self) -> u64 {
        // println!("Converting digits to value: {:?}", self.digits);
        let mut sum = 0;
        let mut factor = 1;
        for digit in self.digits.iter() {
            sum += *digit as u64 * factor;
            factor *= BASE as u64;
        }
        // println!("\tvalue = {sum}");
        sum
    }

    pub fn min_value(num_digits: usize) -> Self {
        let mut digits = vec![0; num_digits];
        if num_digits > 0 {
            digits[num_digits - 1] = 1;
        }
        Self { digits }
    }

    pub fn max_value(num_digits: usize) -> Self {
        let digits = vec![9; num_digits];
        Self { digits }
    }
}

impl<const BASE: u32> RangeDigits<BASE> {
    pub fn from_range(range: Range<BASE>) -> Self {
        let Range { from, to } = range;
        let from = Digits::from_value(from);
        let to = Digits::from_value(to);
        Self { from, to }
    }
}

impl<const BASE: u32> std::str::FromStr for Range<BASE> {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (from, to) = s
            .split_once("-")
            .ok_or_else(|| format!("Invalid range '{s}'"))?;
        let from =
            u64::from_str_radix(from, BASE).map_err(|_| format!("Invalid value '{from}'"))?;
        let to = u64::from_str_radix(to, BASE).map_err(|_| format!("Invalid value '{to}'"))?;
        Ok(Self { from, to })
    }
}

struct DigitsIterator<const BASE: u32> {
    digits: Digits<BASE>,
    complete: bool,
}

impl<const BASE: u32> DigitsIterator<BASE> {
    pub fn new(digits: Digits<BASE>) -> Self {
        Self {
            digits,
            complete: false,
        }
    }
}

impl<const BASE: u32> Iterator for DigitsIterator<BASE> {
    type Item = Digits<BASE>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.complete {
            return None;
        }
        let cur_value = self.digits.clone();
        for digit in self.digits.digits.iter_mut() {
            *digit += 1;
            if (*digit as u32) < BASE {
                return Some(cur_value);
            } else {
                *digit = 0;
            }
        }
        // All digits rolled over, so this is the overflow case.
        self.complete = true;
        Some(cur_value)
    }
}

impl<const BASE: u32> DoubleEndedIterator for DigitsIterator<BASE> {
    fn next_back(&mut self) -> Option<Self::Item> {
        let cur_value = self.digits.clone();
        for digit in self.digits.digits.iter_mut() {
            if *digit > 0 {
                *digit -= 1;
                return Some(cur_value);
            } else {
                *digit = BASE as u8 - 1;
            }
        }
        // All digits rolled under, so this is the underflow case.
        self.complete = true;
        Some(cur_value)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_examples_part1() {
        let get_invalid_ids = |range_string: &str| {
            get_invalid_ids(RangeDigits::<10>::from_range(range_string.parse().unwrap()))
                .collect::<Vec<_>>()
        };
        assert_eq!([11, 22], *get_invalid_ids("11-22"));
        assert_eq!([99], *get_invalid_ids("95-115"));
        assert_eq!([1010], *get_invalid_ids("998-1012"));
        assert_eq!([1188511885], *get_invalid_ids("1188511880-1188511890"));
        assert_eq!([222222], *get_invalid_ids("222220-222224"));
        assert_eq!([] as [u64; 0], *get_invalid_ids("1698522-1698528"));
        assert_eq!([446446], *get_invalid_ids("446443-446449"));
        assert_eq!([38593859], *get_invalid_ids("38593856-38593862"));
        assert_eq!("1227775554", Day02.part1(Example::Example, Debug::NotDebug));
    }

    #[test]
    fn test_real_part1() {
        assert_eq!("29940924880", Day02.part1(Example::Real, Debug::NotDebug));
    }

    #[test]
    fn test_examples_part2() {
        let get_invalid_ids = |range_string: &str| {
            get_invalid_ids_part2(&RangeDigits::<10>::from_range(
                range_string.parse().unwrap(),
            ))
            .collect::<Vec<_>>()
        };
        assert_eq!([11, 22], *get_invalid_ids("11-22"));
        assert_eq!([99, 111], *get_invalid_ids("95-115"));
        assert_eq!([999, 1010], *get_invalid_ids("998-1012"));
        assert_eq!([1188511885], *get_invalid_ids("1188511880-1188511890"));
        assert_eq!([222222], *get_invalid_ids("222220-222224"));
        assert_eq!([] as [u64; 0], *get_invalid_ids("1698522-1698528"));
        assert_eq!([446446], *get_invalid_ids("446443-446449"));
        assert_eq!([38593859], *get_invalid_ids("38593856-38593862"));
        assert_eq!([565656], *get_invalid_ids("565653-565659"));
        assert_eq!([824824824], *get_invalid_ids("824824821-824824827"));
        assert_eq!([2121212121], *get_invalid_ids("2121212118-2121212124"));
        assert_eq!("4174379265", Day02.part2(Example::Example, Debug::NotDebug));
    }

    #[test]
    fn test_real_part2() {
        assert_eq!("48631958998", Day02.part2(Example::Real, Debug::NotDebug));
    }
}
