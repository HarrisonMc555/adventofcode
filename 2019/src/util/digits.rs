use num_traits::*;
use std::fmt::Debug;

pub trait DigitNum:
    Num + NumRef + NumOps + NumAssignOps + FromPrimitive + ToPrimitive + Ord + Debug + Clone
{
}

impl<T> DigitNum for T where
    T: Num + NumRef + NumOps + NumAssignOps + FromPrimitive + ToPrimitive + Ord + Debug + Clone
{
}

pub struct Digits<T> {
    n: T,
    divisor: T,
    base: T,
}

pub struct DigitsRev<T> {
    n: T,
    base: T,
    started: bool,
}

impl<T> Digits<T>
where
    T: DigitNum,
{
    pub fn decimal(n: T) -> Self {
        Self::new(n, 10)
    }

    #[allow(dead_code)]
    pub fn binary(n: T) -> Self {
        Self::new(n, 2)
    }

    #[allow(dead_code)]
    pub fn octal(n: T) -> Self {
        Self::new(n, 8)
    }

    #[allow(dead_code)]
    pub fn hex(n: T) -> Self {
        Self::new(n, 16)
    }

    fn new(n: T, base: u8) -> Self {
        let base = T::from_u8(base).unwrap();
        let mut divisor = T::one();
        while n >= divisor.clone() * &base {
            divisor *= base.clone();
        }
        Digits { n, divisor, base }
    }
}

impl<T> DigitsRev<T>
where
    T: DigitNum,
{
    pub fn decimal(n: T) -> Self {
        Self::new(n, 10)
    }

    #[allow(dead_code)]
    pub fn binary(n: T) -> Self {
        Self::new(n, 2)
    }

    #[allow(dead_code)]
    pub fn octal(n: T) -> Self {
        Self::new(n, 8)
    }

    #[allow(dead_code)]
    pub fn hex(n: T) -> Self {
        Self::new(n, 16)
    }

    fn new(n: T, base: u8) -> Self {
        let base = T::from_u8(base).unwrap();
        Self {
            n,
            base,
            started: false,
        }
    }
}

impl<T> Iterator for Digits<T>
where
    T: DigitNum,
{
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        if self.divisor == zero() {
            return None;
        }
        let v = (self.n.clone() / self.divisor.clone()).to_u8().unwrap();
        self.n %= self.divisor.clone();
        self.divisor /= self.base.clone();
        Some(v)
    }
}

impl<T> Iterator for DigitsRev<T>
where
    T: DigitNum,
{
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        if self.n == zero() {
            if self.started {
                return None;
            } else {
                self.started = true;
                return Some(0);
            }
        }
        self.started = true;
        let v = (self.n.clone() % self.base.clone()).to_u8().unwrap();
        self.n /= self.base.clone();
        Some(v)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn digits_normal() {
        let num = 785;
        let mut digits = Digits::decimal(num);
        assert_eq!(digits.next(), Some(7));
        assert_eq!(digits.next(), Some(8));
        assert_eq!(digits.next(), Some(5));
        assert_eq!(digits.next(), None);
    }

    #[test]
    fn digits_single_digit() {
        let num = 7;
        let mut digits = Digits::decimal(num);
        assert_eq!(digits.next(), Some(7));
        assert_eq!(digits.next(), None);
    }

    #[test]
    fn digits_zero() {
        let num = 0;
        let mut digits = Digits::decimal(num);
        assert_eq!(digits.next(), Some(0));
        assert_eq!(digits.next(), None);
    }

    #[test]
    fn digits_rev_normal() {
        let num = 785;
        let mut digits = DigitsRev::decimal(num);
        assert_eq!(digits.next(), Some(5));
        assert_eq!(digits.next(), Some(8));
        assert_eq!(digits.next(), Some(7));
        assert_eq!(digits.next(), None);
    }

    #[test]
    fn digits_rev_single_digit() {
        let num = 7;
        let mut digits = DigitsRev::decimal(num);
        assert_eq!(digits.next(), Some(7));
        assert_eq!(digits.next(), None);
    }

    #[test]
    fn digits_rev_zero() {
        let num = 0;
        let mut digits = DigitsRev::decimal(num);
        assert_eq!(digits.next(), Some(0));
        assert_eq!(digits.next(), None);
    }

    #[test]
    fn binary_normal() {
        let num = 11;
        let mut digits = Digits::binary(num);
        assert_eq!(digits.next(), Some(1));
        assert_eq!(digits.next(), Some(0));
        assert_eq!(digits.next(), Some(1));
        assert_eq!(digits.next(), Some(1));
        assert_eq!(digits.next(), None);
    }

    #[test]
    fn binary_single_digit() {
        let num = 1;
        let mut digits = Digits::binary(num);
        assert_eq!(digits.next(), Some(1));
        assert_eq!(digits.next(), None);
    }

    #[test]
    fn binary_zero() {
        let num = 0;
        let mut digits = Digits::binary(num);
        assert_eq!(digits.next(), Some(0));
        assert_eq!(digits.next(), None);
    }

    #[test]
    fn binary_rev_normal() {
        let num = 11;
        let mut digits = DigitsRev::binary(num);
        assert_eq!(digits.next(), Some(1));
        assert_eq!(digits.next(), Some(1));
        assert_eq!(digits.next(), Some(0));
        assert_eq!(digits.next(), Some(1));
        assert_eq!(digits.next(), None);
    }

    #[test]
    fn binary_rev_single_digit() {
        let num = 1;
        let mut digits = DigitsRev::binary(num);
        assert_eq!(digits.next(), Some(1));
        assert_eq!(digits.next(), None);
    }

    #[test]
    fn binary_rev_zero() {
        let num = 0;
        let mut digits = DigitsRev::binary(num);
        assert_eq!(digits.next(), Some(0));
        assert_eq!(digits.next(), None);
    }

    #[test]
    fn hex_normal() {
        let num = (16 * 2) + 11;
        let mut digits = Digits::hex(num);
        assert_eq!(digits.next(), Some(2));
        assert_eq!(digits.next(), Some(11));
        assert_eq!(digits.next(), None);
    }

    #[test]
    fn hex_single_digit() {
        let num = 12;
        let mut digits = Digits::hex(num);
        assert_eq!(digits.next(), Some(12));
        assert_eq!(digits.next(), None);
    }

    #[test]
    fn hex_zero() {
        let num = 0;
        let mut digits = Digits::hex(num);
        assert_eq!(digits.next(), Some(0));
        assert_eq!(digits.next(), None);
    }

    #[test]
    fn hex_rev_normal() {
        let num = (16 * 2) + 11;
        let mut digits = DigitsRev::hex(num);
        assert_eq!(digits.next(), Some(11));
        assert_eq!(digits.next(), Some(2));
        assert_eq!(digits.next(), None);
    }

    #[test]
    fn hex_rev_single_digit() {
        let num = 12;
        let mut digits = DigitsRev::hex(num);
        assert_eq!(digits.next(), Some(12));
        assert_eq!(digits.next(), None);
    }

    #[test]
    fn hex_rev_zero() {
        let num = 0;
        let mut digits = DigitsRev::hex(num);
        assert_eq!(digits.next(), Some(0));
        assert_eq!(digits.next(), None);
    }
}
