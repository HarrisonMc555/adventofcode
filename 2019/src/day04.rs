const INPUT: &str = include_str!("../static/day04.txt");

type Result<T> = std::result::Result<T, ()>;
type Value = u32;

pub fn main() {
    let answer1 = solve1(INPUT).unwrap();
    println!("{}", answer1);
}

fn solve1(input: &str) -> Result<usize> {
    let (beg, end) = parse_input(input)?;
    let count = (beg..=end).filter(|n| meets_criteria(*n)).count();
    Ok(count)
}

fn meets_criteria(password: Value) -> bool {
    let digits = password.digits().collect::<Vec<_>>();
    is_increasing(&digits) && has_repeat(&digits)
}

fn is_increasing(nums: &[u8]) -> bool {
    windows2(nums).all(|(d1, d2)| d1 <= d2)
}

fn has_repeat<T>(slice: &[T]) -> bool
where
    T: PartialEq,
{
    windows2(slice).any(|(v1, v2)| v1 == v2)
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

struct Digits {
    n: Value,
    divisor: Value,
}

impl Digits {
    fn new(n: Value) -> Self {
        let mut divisor = 1;
        while n >= divisor * 10 {
            divisor *= 10;
        }

        Digits {
            n: n,
            divisor: divisor,
        }
    }
}

impl Iterator for Digits {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        if self.divisor == 0 {
            None
        } else {
            let v = (self.n / self.divisor) as u8;
            self.n %= self.divisor;
            self.divisor /= 10;
            Some(v)
        }
    }
}

trait ToDigits {
    fn digits(&self) -> Digits;
}

impl ToDigits for Value {
    fn digits(&self) -> Digits {
        Digits::new(*self)
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
    fn digits_normal() {
        let num = 785;
        let mut digits = Digits::new(num);
        assert_eq!(digits.next(), Some(7));
        assert_eq!(digits.next(), Some(8));
        assert_eq!(digits.next(), Some(5));
        assert_eq!(digits.next(), None);
    }

    #[test]
    fn digits_single_digit() {
        let num = 7;
        let mut digits = Digits::new(num);
        assert_eq!(digits.next(), Some(7));
        assert_eq!(digits.next(), None);
    }

    #[test]
    fn digits_zero() {
        let num = 0;
        let mut digits = Digits::new(num);
        assert_eq!(digits.next(), Some(0));
        assert_eq!(digits.next(), None);
    }

    #[test]
    fn meets_criteria_tests() {
        assert!(meets_criteria(111111));
        assert!(!meets_criteria(223450));
        assert!(!meets_criteria(123789));
    }
}
