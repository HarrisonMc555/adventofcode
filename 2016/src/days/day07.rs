use crate::days::{Day, Debug, Example, Part};
use std::collections::HashSet;

pub struct Day07;

impl Day for Day07 {
    fn number(&self) -> u32 {
        7
    }

    fn run(&self, part: Part, example: Example, debug: Debug) {
        let answer = match part {
            Part::Part1 => self.part1(example, debug),
            Part::Part2 => self.part2(example, debug),
        };
        println!("{}", answer);
    }
}

impl Day07 {
    fn part1(&self, example: Example, _debug: Debug) -> usize {
        self.get_lines(example)
            .into_iter()
            .filter(|line| supports_tls(line))
            .count()
    }

    fn part2(&self, example: Example, _debug: Debug) -> usize {
        self.get_lines(example)
            .into_iter()
            .filter(|line| supports_ssl(line))
            .count()
    }
}

const ABBA_LEN: usize = 4;
fn supports_tls<T: AsRef<str>>(ip_addr: T) -> bool {
    let mut in_hypernet_sequence = false;
    let chars = ip_addr.as_ref().chars().collect::<Vec<_>>();
    let mut found_abba = false;
    for window in chars.windows(ABBA_LEN) {
        match window[0] {
            '[' => in_hypernet_sequence = true,
            ']' => in_hypernet_sequence = false,
            _ => {}
        }
        if is_abba(window.try_into().unwrap()) {
            if in_hypernet_sequence {
                return false;
            } else {
                found_abba = true;
            }
        }
    }
    found_abba
}

fn is_abba(window: &[char; ABBA_LEN]) -> bool {
    let [a, b, c, d] = window;
    a == d && b == c && a != b
}

const ABA_LEN: usize = 3;
fn supports_ssl<T: AsRef<str>>(ip_addr: T) -> bool {
    let mut in_hypernet_sequence = false;
    let chars = ip_addr.as_ref().chars().collect::<Vec<_>>();
    let mut abas = HashSet::new();
    let mut babs = HashSet::new();
    for window in chars.windows(ABA_LEN) {
        match window[0] {
            '[' => in_hypernet_sequence = true,
            ']' => in_hypernet_sequence = false,
            _ => {}
        }
        let window = window.try_into().unwrap();
        if is_aba(window) {
            if in_hypernet_sequence {
                if abas.contains(&create_bab(window)) {
                    return true;
                }
                babs.insert(window);
            } else {
                if babs.contains(&create_bab(window)) {
                    return true;
                }
                abas.insert(window);
            }
        }
    }
    false
    // abas.into_iter().map(create_bab).any(|bab| babs.contains(&bab))
}

fn is_aba(window: &[char; ABA_LEN]) -> bool {
    let [a, b, c] = window;
    a == c && a != b
}

fn create_bab(window: &[char; ABA_LEN]) -> [char; ABA_LEN] {
    let [a, b, _] = window;
    [*b, *a, *b]
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_examples_part1() {
        assert!(supports_tls("abba[mnop]qrst"));
        assert!(!supports_tls("abcd[bddb]xyyx"));
        assert!(!supports_tls("aaaa[qwer]tyui"));
        assert!(supports_tls("ioxxoj[asdfgh]zxcvbn"));
    }

    #[test]
    fn test_real_part1() {
        assert_eq!(110, Day07.part1(Example::Real, Debug::NotDebug));
    }

    #[test]
    fn test_examples_part2() {
        assert!(supports_ssl("aba[bab]xyz"));
        assert!(!supports_ssl("xyx[xyx]xyx"));
        assert!(supports_ssl("aaa[kek]eke"));
        assert!(supports_ssl("zazbz[bzb]cdb"));
    }

    #[test]
    fn test_real_part2() {
        assert_eq!(242, Day07.part2(Example::Real, Debug::NotDebug));
    }
}
