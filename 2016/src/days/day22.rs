use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;
use std::str::FromStr;

use crate::days::{Day, Debug, Example, Part};

const DEBUG: bool = true;

const NUM_HEADER_LINES: usize = 2;

pub struct Day22;

impl Day for Day22 {
    fn number(&self) -> u32 {
        22
    }

    fn run(&self, part: Part, example: Example, debug: Debug) {
        let answer = match part {
            Part::Part1 => self.part1(example, debug),
            Part::Part2 => self.part2(example, debug),
        };
        println!("{}", answer);
    }
}

impl Day22 {
    fn part1(&self, example: Example, _debug: Debug) -> usize {
        let nodes = NodeInfo::parse_lines(&self.read_file(example)).unwrap();
        count_viable_pairs(&nodes)
    }

    fn part2(&self, _example: Example, _debug: Debug) -> usize {
        todo!()
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct NodeInfo {
    x: usize,
    y: usize,
    size: usize,
    used: usize,
    avail: usize,
}

fn count_viable_pairs(nodes: &[NodeInfo]) -> usize {
    count_overlap_pairs(
        nodes.iter().map(|n| n.used).filter(|used| *used != 0),
        nodes.iter().map(|n| n.avail),
    )
}

fn count_overlap_pairs<IF, IT>(from: IF, into: IT) -> usize
where
    IF: IntoIterator<Item = usize>,
    IT: IntoIterator<Item = usize>,
{
    let from = from.into_iter().sorted_unstable();
    let into = into
        .into_iter()
        .sorted_unstable_by_key(|&x| std::cmp::Reverse(x))
        .collect::<Vec<_>>();
    let mut count = 0;
    let mut max_index = into.len();
    let mut prev = None;
    for val_from in from {
        if let Some((prev_val, prev_count)) = prev {
            if prev_val == val_from {
                count += prev_count;
                continue;
            }
        };
        debug_println!(
            "\tHow many does {} fit into? Max index is {} ({:?})",
            val_from,
            max_index,
            into.get(max_index)
        );
        max_index = into[..max_index].partition_point(|&val_into| val_from <= val_into);
        debug_println!(
            "\t\tFits {}. {:?} < {} <= {:?}",
            max_index,
            &into[max_index..],
            val_from,
            &into[..max_index],
        );
        if max_index == 0 {
            break;
        }
        count += max_index;
        prev = Some((val_from, max_index));
    }
    count
}

impl NodeInfo {
    pub fn parse_lines(s: &str) -> Result<Vec<Self>, <Self as FromStr>::Err> {
        s.lines()
            .skip(NUM_HEADER_LINES)
            .map(Self::from_str)
            .collect()
    }
}

impl FromStr for NodeInfo {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex =
                Regex::new(r"^/dev/grid/node-x(\d+)-y(\d+) +(\d+)T +(\d+)T +(\d+)T+ +(\d+)%$")
                    .unwrap();
        }
        let caps = RE
            .captures(s)
            .ok_or_else(|| format!("Invalid line: \"{}\"", s))?;
        let parse_usize = |group| caps.get(group).unwrap().as_str().parse::<usize>().unwrap();
        let x = parse_usize(1);
        let y = parse_usize(2);
        let size = parse_usize(3);
        let used = parse_usize(4);
        let avail = parse_usize(5);
        Ok(Self {
            x,
            y,
            size,
            used,
            avail,
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse() {
        let line = "/dev/grid/node-x0-y0     92T   68T    24T   73%";
        let actual = line.parse();
        let expected = NodeInfo {
            x: 0,
            y: 0,
            size: 92,
            used: 68,
            avail: 24,
        };
        assert_eq!(Ok(expected.clone()), actual);

        let lines = "root@ebhq-gridcenter# df -h\n\
            Filesystem              Size  Used  Avail  Use%\n\
            /dev/grid/node-x0-y0     92T   68T    24T   73%\n\
            /dev/grid/node-x0-y1     90T   68T    22T   75%";
        let actual = NodeInfo::parse_lines(lines);
        let expected = vec![
            expected,
            NodeInfo {
                x: 0,
                y: 1,
                size: 90,
                used: 68,
                avail: 22,
            },
        ];
        assert_eq!(Ok(expected), actual);
    }

    #[test]
    fn test_count_overlap_pairs() {
        assert_eq!(0, count_overlap_pairs(&[], &[]));
        assert_eq!(0, count_overlap_pairs(&[1], &[]));
        assert_eq!(0, count_overlap_pairs(&[], &[1]));
        assert_eq!(1, count_overlap_pairs(&[0], &[1]));
        assert_eq!(1, count_overlap_pairs(&[1], &[1]));
        assert_eq!(0, count_overlap_pairs(&[2], &[1]));
        assert_eq!(2, count_overlap_pairs(&[1], &[1, 2]));
        assert_eq!(1, count_overlap_pairs(&[2], &[1, 2]));
        assert_eq!(0, count_overlap_pairs(&[3], &[1, 2]));
        assert_eq!(
            6 + 4 + 2,
            count_overlap_pairs(&[1, 4, 6], &[1, 3, 4, 5, 6, 7])
        );
    }

    fn count_overlap_pairs(from: &[usize], into: &[usize]) -> usize {
        super::count_overlap_pairs(from.iter().copied(), into.iter().copied())
    }

    #[test]
    fn test_examples_part1() {}

    #[test]
    fn test_real_part1() {
        // assert_eq!(0, Day22.part1(Example::Real, Debug::NotDebug));
    }

    #[test]
    fn test_examples_part2() {}

    #[test]
    fn test_real_part2() {
        // assert_eq!(0, Day22.part2(Example::Real, Debug::NotDebug));
    }
}
