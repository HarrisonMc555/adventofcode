use crate::util::digits::Digits;

const INPUT: &str = include_str!("../../static/day04.txt");

type Result<T> = std::result::Result<T, ()>;
type Value = u32;

pub fn main() {
    let answer1 = solve1(INPUT).unwrap();
    println!("{}", answer1);
    let answer2 = solve2(INPUT).unwrap();
    println!("{}", answer2);
}

fn solve1(input: &str) -> Result<usize> {
    let (beg, end) = parse_input(input)?;
    let count = (beg..=end).filter(|&n| meets_criteria1(n)).count();
    Ok(count)
}

fn solve2(input: &str) -> Result<usize> {
    let (beg, end) = parse_input(input)?;
    let count = (beg..=end).filter(|&n| meets_criteria2(n)).count();
    Ok(count)
}

fn meets_criteria1(password: Value) -> bool {
    let digits = Digits::decimal(password).collect::<Vec<_>>();
    is_increasing(&digits) && has_repeat(&digits, 2)
}

fn meets_criteria2(password: Value) -> bool {
    let digits = Digits::decimal(password).collect::<Vec<_>>();
    is_increasing(&digits) && has_repeat_exact(&digits, 2)
}

fn is_increasing(nums: &[u8]) -> bool {
    windows2(nums).all(|(d1, d2)| d1 <= d2)
}

fn has_repeat<T>(slice: &[T], length: usize) -> bool
where
    T: PartialEq,
{
    slice.windows(length).any(|window| all_same(window))
}

fn has_repeat_exact<T>(slice: &[T], length: usize) -> bool
where
    T: PartialEq,
{
    Group::new(slice).any(|(_, group_len)| group_len == length)
}

fn all_same<T>(slice: &[T]) -> bool
where
    T: PartialEq,
{
    windows2(slice).all(|(v1, v2)| v1 == v2)
}

fn windows2<T>(slice: &[T]) -> impl Iterator<Item = (&T, &T)> {
    slice.windows(2).map(|s| (&s[0], &s[1]))
}

fn parse_input(input: &str) -> Result<(Value, Value)> {
    let nums = input
        .trim()
        .split('-')
        .map(|w| w.parse().map_err(|_| ()))
        .collect::<Result<Vec<_>>>()?;
    match nums.as_slice() {
        [n1, n2] => Ok((*n1, *n2)),
        _ => Err(()),
    }
}

struct Group<'a, T> {
    slice: &'a [T],
    index: usize,
}

impl<'a, T> Group<'a, T> {
    pub fn new(slice: &'a [T]) -> Self {
        Group { slice, index: 0 }
    }
}

impl<'a, T> Iterator for Group<'a, T>
where
    T: PartialEq,
{
    type Item = (&'a T, usize);

    fn next(&mut self) -> Option<Self::Item> {
        let first = &self.slice.get(self.index)?;
        let count = self.slice[self.index..]
            .iter()
            .take_while(|&v| v == *first)
            .count();
        self.index += count;
        Some((first, count))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn windows2_normal() {
        let slice = [1, 2, 4, 8];
        let mut iter = windows2(&slice);
        assert_eq!(iter.next(), Some((&1, &2)));
        assert_eq!(iter.next(), Some((&2, &4)));
        assert_eq!(iter.next(), Some((&4, &8)));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn windows2_singleton() {
        let slice = [1];
        let mut iter = windows2(&slice);
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn windows2_empty() {
        let slice: [u32; 0] = [];
        let mut iter = windows2(&slice);
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn meets_criteria_tests() {
        assert!(meets_criteria1(111111));
        assert!(!meets_criteria1(223450));
        assert!(!meets_criteria1(123789));
    }

    #[test]
    fn answer01a() {
        assert_eq!(solve1(INPUT), Ok(579));
    }

    #[test]
    fn answer01b() {
        assert_eq!(solve2(INPUT), Ok(358));
    }
}
