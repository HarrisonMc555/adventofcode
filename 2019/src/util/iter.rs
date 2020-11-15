pub trait IteratorExtensions: Iterator {
    fn duplicate_values(self, num_duplicates: usize) -> DuplicateValues<Self, Self::Item>
    where
        Self::Item: Clone,
        Self: Sized,
    {
        DuplicateValues {
            iter: self,
            cached_value: None,
            num_duplicates,
            cur_count: 0,
        }
    }
}

impl<I: Iterator> IteratorExtensions for I {}

pub struct DuplicateValues<I, T> {
    iter: I,
    cached_value: Option<T>,
    num_duplicates: usize,
    cur_count: usize,
}

impl<I, T> Iterator for DuplicateValues<I, T>
where
    I: Iterator<Item = T>,
    T: Clone,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        // Retrieve the first value
        if self.cached_value.is_none() || self.cur_count >= self.num_duplicates {
            self.cur_count = 0;
            self.cached_value = Some(self.iter.next()?);
        }
        let value = self.cached_value.as_ref()?;
        self.cur_count += 1;
        Some(value.clone())
    }
}

mod test {
    use super::IteratorExtensions;

    #[test]
    fn empty() {
        let iter = std::iter::empty::<i32>();
        let mut duplicated = iter.duplicate_values(3);
        assert_eq!(duplicated.next(), None);
        assert_eq!(duplicated.next(), None);
        assert_eq!(duplicated.next(), None);
    }

    #[test]
    fn once() {
        let iter = [1, 7, -3].iter();
        let mut duplicated = iter.duplicate_values(1);
        assert_eq!(duplicated.next(), Some(&1));
        assert_eq!(duplicated.next(), Some(&7));
        assert_eq!(duplicated.next(), Some(&-3));
        assert_eq!(duplicated.next(), None);
    }

    #[test]
    fn thrice() {
        let iter = [1, 7, -3].iter();
        let mut duplicated = iter.duplicate_values(3);
        assert_eq!(duplicated.next(), Some(&1));
        assert_eq!(duplicated.next(), Some(&1));
        assert_eq!(duplicated.next(), Some(&1));
        assert_eq!(duplicated.next(), Some(&7));
        assert_eq!(duplicated.next(), Some(&7));
        assert_eq!(duplicated.next(), Some(&7));
        assert_eq!(duplicated.next(), Some(&-3));
        assert_eq!(duplicated.next(), Some(&-3));
        assert_eq!(duplicated.next(), Some(&-3));
        assert_eq!(duplicated.next(), None);
    }
}
