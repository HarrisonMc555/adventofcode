use itertools::Itertools;

use crate::days::{Day, Debug, Example, Part};
use crate::debug_println;

const DEBUG: Debug = Debug::NotDebug;

pub struct Day02;

const MAX_DIFF: isize = 3;

impl Day for Day02 {
    fn number(&self) -> u32 {
        2
    }

    fn part1(&self, example: Example, _debug: Debug) -> String {
        solve1(parse(self.get_lines(Part::Part1, example)).unwrap()).to_string()
    }

    fn part2(&self, example: Example, _debug: Debug) -> String {
        solve2(parse(self.get_lines(Part::Part2, example)).unwrap()).to_string()
    }
}

fn parse(lines: Vec<String>) -> Option<Vec<Vec<isize>>> {
    lines.into_iter().map(|line| parse_line(&line)).collect()
}

fn parse_line(line: &str) -> Option<Vec<isize>> {
    line.split_ascii_whitespace().map(|word| word.parse().ok()).collect()
}

fn solve1(reports: Vec<Vec<isize>>) -> usize {
    reports.into_iter().filter(|report| is_safe(report)).count()
}

fn solve2(reports: Vec<Vec<isize>>) -> usize {
    for report in reports.iter() {
        debug_println!(DEBUG, "Testing report {report:?}...");
        debug_println!(DEBUG, "Report {report:?} is {}", if is_safe2(report) { "safe" } else { "NOT safe" });
        debug_println!(DEBUG);
    }
    reports.into_iter().filter(|report| is_safe2(report)).count()
}

fn is_safe(report: &[isize]) -> bool {
    let diffs = report.iter().tuple_windows().map(|(x, y)| y - x).collect::<Vec<_>>();
    diffs.iter().all(|&diff| 0 < diff && diff <= MAX_DIFF) || diffs.iter().all(|&diff| -MAX_DIFF <= diff && diff < 0)
}

fn is_safe2(report: &[isize]) -> bool {
    if is_safe2_naive(report) != is_safe2_smart(report) {
        debug_println!(DEBUG, "Discrepancy for report: {report:?}");
        debug_println!(DEBUG, "\tNaive says {}", is_safe2_naive(report));
        debug_println!(DEBUG, "\tSmart says {}", is_safe2_smart(report));
    }
    is_safe2_naive(report)
}

fn is_safe2_smart(report: &[isize]) -> bool {
    let diffs = report.iter().tuple_windows().map(|(x, y)| y - x).collect::<Vec<_>>();
    debug_println!(DEBUG, "\tTesting with diffs {diffs:?}");
    debug_println!(DEBUG, "\tTesting incremeting...");
    if is_safe2_inc_dec_try2(&diffs, IncDec::Inc) {
        return true;
    }
    debug_println!(DEBUG, "\tTesting decrementing...");
    if is_safe2_inc_dec_try2(&diffs, IncDec::Dec) {
        return true;
    }
    false
    // is_safe2_inc_dec(&diffs, IncDec::Inc) || is_safe2_inc_dec(&diffs, IncDec::Dec)
}

fn is_safe2_naive(report: &[isize]) -> bool {
    if is_safe(report) {
        return true;
    }
    for index in 0..report.len() {
        let mut new_report = report.to_vec();
        new_report.remove(index);
        if is_safe(&new_report) {
            return true;
        }
    }
    false
}

#[derive(Debug, Hash, Eq, PartialEq, Ord, PartialOrd, Copy, Clone)]
enum IncDec {
    Inc,
    Dec,
}

fn is_safe2_inc_dec(diffs: &[isize], inc_dec: IncDec) -> bool {
    let mut discarded_bad = false;
    if let [first, .., last] = diffs {
        if !is_safe_diff_inc_dec(*first, inc_dec) {
            debug_println!(DEBUG, "\t\tDiscarding first diff {first}");
            discarded_bad = true;
        }
        if !is_safe_diff_inc_dec(*last, inc_dec) {
            if discarded_bad {
                debug_println!(DEBUG, "\t\tFirst and last diff are bad");
                return false;
            }
            debug_println!(DEBUG, "\t\tDiscarding last diff {last}");
            discarded_bad = true;
        }
    };
    for (&prev_diff, &diff, &next_diff) in diffs.iter().tuple_windows() {
        debug_println!(DEBUG, "\t\t\tTesting diffs {prev_diff}, {diff}, {next_diff}");
        if is_safe_diff_inc_dec(diff, inc_dec) {
            continue;
        }
        if discarded_bad {
            debug_println!(DEBUG, "\t\tEncountered second bad diff {diff}");
            return false;
        }
        if !is_safe_diff_inc_dec(prev_diff + diff, inc_dec) && !is_safe_diff_inc_dec(diff + next_diff, inc_dec) {
            debug_println!(DEBUG, "\t\tDiff {diff} is bad, and even with combining with previous diff {prev_diff} or next diff {next_diff}");
            return false;
        }
        discarded_bad = true;
    }
    true
}

fn is_safe2_inc_dec_try2(diffs: &[isize], inc_dec: IncDec) -> bool {
    let mut discarded_index = None;
    for (index, (&prev_diff, &diff, &next_diff)) in diffs.iter().tuple_windows().enumerate() {
        debug_println!(DEBUG, "\t\t\tTesting diffs {prev_diff}, {diff}, {next_diff}");
        if is_safe_diff_inc_dec(diff, inc_dec) {
            continue;
        }
        if let Some(first_bad_index) = discarded_index {
            debug_println!(DEBUG, "\t\tTwo bad diff indices: {first_bad_index} and {index} ({diff})");
            return false;
        }
        if !is_safe_diff_inc_dec(prev_diff + diff, inc_dec) && !is_safe_diff_inc_dec(diff + next_diff, inc_dec) {
            debug_println!(DEBUG, "\t\tDiff {diff} is bad, and even with combining with previous diff {prev_diff} or next diff {next_diff}");
            return false;
        }
        discarded_index = Some(index);
    }
    let [first, .., last] = diffs else {
        return true;
    };
    if !is_safe_diff_inc_dec(*first, inc_dec) {
        match discarded_index {
            Some(i) if i != 0 => {
                debug_println!(DEBUG, "\t\tFirst diff {first} is not safe, and we did not discard the \"first\" diff (discarded {i})");
                return false;
            },
            _ => {},
        }
    }
    if !is_safe_diff_inc_dec(*last, inc_dec) {
        match discarded_index {
            Some(i) if i != diffs.len() - 3 => {
                debug_println!(DEBUG, "\t\tLast diff {last} is not safe, and we did not discard the \"last\" diff (discarded {i})");
                return false;
            },
            _ => {},
        }
    }
    true
}

fn is_safe_diff_inc_dec(diff: isize, inc_dec: IncDec) -> bool {
    // match inc_dec {
    //     IncDec::Inc => 0 < diff && diff <= MAX_DIFF,
    //     IncDec::Dec => 0 > diff && diff >= -MAX_DIFF,
    // }
    let safe = match inc_dec {
        IncDec::Inc => 0 < diff && diff <= MAX_DIFF,
        IncDec::Dec => 0 > diff && diff >= -MAX_DIFF,
    };
    debug_println!(DEBUG, "\t\t\t\tDiff {diff} is {} for {inc_dec:?}", if safe { "safe" } else { "NOT safe" });
    safe
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    #[ignore]
    fn test_examples_part1() {
        assert_eq!("0", Day02.part1(Example::Example, Debug::NotDebug));
    }

    #[test]
    #[ignore]
    fn test_real_part1() {
        assert_eq!("0", Day02.part1(Example::Real, Debug::NotDebug));
    }

    #[test]
    #[ignore]
    fn test_examples_part2() {
        assert_eq!("0", Day02.part2(Example::Example, Debug::NotDebug));
    }

    #[test]
    #[ignore]
    fn test_real_part2() {
        assert_eq!("0", Day02.part2(Example::Real, Debug::NotDebug));
    }
}
