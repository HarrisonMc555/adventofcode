use lazy_static::lazy_static;
use regex::Regex;

use crate::days::{Day, Debug, Example, Part};
use crate::debug_println;

const DEBUG: bool = true;

pub struct Day15;

impl Day for Day15 {
    fn number(&self) -> u32 {
        15
    }

    fn run(&self, part: Part, example: Example, debug: Debug) {
        let answer = match part {
            Part::Part1 => self.part1(example, debug),
            Part::Part2 => self.part2(example, debug),
        };
        println!("{}", answer);
    }
}

impl Day15 {
    fn part1(&self, example: Example, _debug: Debug) -> usize {
        let sensors = parse_sensors(&self.read_file(example)).unwrap();
        count_eliminated_positions_in_row(&sensors, ROW)
    }

    fn part2(&self, _example: Example, _debug: Debug) -> usize {
        todo!()
    }
}

const ROW: isize = 2000000;

#[derive(Debug, Hash, Clone, Eq, PartialEq)]
struct Sensor {
    position: Point,
    closest_beacon: Point,
}

#[derive(Debug, Hash, Clone, Eq, PartialEq)]
struct Point {
    x: isize,
    y: isize,
}

/// Represents an inclusive range, equivalent to `start..=end`
#[derive(Debug, Hash, Clone, Eq, PartialEq)]
struct Range {
    start: isize,
    end: isize,
}

#[derive(Debug, Hash, Clone, Eq, PartialEq)]
enum RangeMergeResult {
    /// The given ranges could not be merged
    Unmerged(Range, Range),
    /// The given ranges were merged into this result
    Merged(Range),
}

fn count_eliminated_positions_in_row(sensors: &[Sensor], row: isize) -> usize {
    debug_println!("Counting eliminated positions in row {}", row);
    let eliminated_ranges = sensors
        .iter()
        .filter_map(|sensor| sensor.get_eliminated_range_from_row(row))
        .collect::<Vec<_>>();
    debug_println!("Eliminated ranges from sensors:");
    for range in eliminated_ranges.iter() {
        debug_println!("\t{}..={}", range.start, range.end);
    }
    let eliminated_ranges = merge_ranges(eliminated_ranges);
    debug_println!("After merging ranges, eliminated ranges from sensors:");
    for range in eliminated_ranges.iter() {
        debug_println!("\t{}..={}", range.start, range.end);
    }
    let total_eliminated_positions: usize = eliminated_ranges.iter().map(Range::len).sum();
    debug_println!("Total eliminated positions: {}", total_eliminated_positions);
    let beacon_columns = sensors
        .iter()
        .map(|sensor| &sensor.closest_beacon)
        .filter(|position| position.y == row)
        .map(|position| position.x)
        .collect();
    debug_println!("Beacon columns in row {}: {:?}", row, beacon_columns);
    let num_beacons_in_ranges = count_values_in_ranges(beacon_columns, eliminated_ranges);
    debug_println!("Number of beacons in eliminated ranges: {}", num_beacons_in_ranges);
    total_eliminated_positions.saturating_sub(num_beacons_in_ranges)
}

fn count_values_in_ranges(mut values: Vec<isize>, mut sorted_merged_ranges: Vec<Range>) -> usize {
    values.sort_unstable();
    values.dedup();
    sorted_merged_ranges.sort_unstable_by_key(|range| range.start);

    let mut value_iter = values.into_iter();
    let mut range_iter = sorted_merged_ranges.iter();

    let Some(mut value) = value_iter.next() else {
        return 0;
    };
    let Some(mut range) = range_iter.next() else {
        return 0;
    };

    let mut num_values_in_ranges = 0;
    loop {
        if value < range.start {
            match value_iter.next() {
                Some(v) => value = v,
                None => break,
            }
        } else if value <= range.end {
            num_values_in_ranges += 1;
            match value_iter.next() {
                Some(v) => value = v,
                None => break,
            }
        } else {
            match range_iter.next() {
                Some(r) => range = r,
                None => break,
            };
        }
    }
    num_values_in_ranges
}

impl Sensor {
    fn get_eliminated_range_from_row(&self, row: isize) -> Option<Range> {
        let distance_to_beacon = self.calc_distance_to_beacon();
        let distance_to_row = (self.position.y - row).abs();
        let radius = distance_to_beacon - distance_to_row;
        if radius < 0 {
            return None;
        }
        let mid_x = self.position.x;
        let min_x = mid_x - radius;
        let max_x = mid_x + radius;
        Some(Range::new(min_x, max_x))
    }

    fn calc_distance_to_beacon(&self) -> isize {
        (self.closest_beacon.x - self.position.x).abs()
            + (self.closest_beacon.y - self.position.y).abs()
    }
}

fn merge_ranges(mut ranges: Vec<Range>) -> Vec<Range> {
    let mut result = Vec::new();
    // Sort backwards so we can pop from back
    ranges.sort_unstable_by_key(|range| -range.start);
    let Some(mut cur_range) = ranges.pop() else {
        return result;
    };
    for next_range in ranges.into_iter().rev() {
        match cur_range.merge(next_range) {
            RangeMergeResult::Unmerged(prev_range, next_range) => {
                result.push(prev_range);
                cur_range = next_range;
            }
            RangeMergeResult::Merged(merged_range) => {
                cur_range = merged_range;
            }
        }
    }
    result.push(cur_range);
    result
}

impl Range {
    fn new(start: isize, end: isize) -> Self {
        Range { start, end }
    }

    fn len(&self) -> usize {
        ((self.end - self.start).abs() + 1) as usize
    }

    fn merge(self, other_range: Range) -> RangeMergeResult {
        if self.overlaps(&other_range) {
            let start = self.start.min(other_range.start);
            let end = self.end.max(other_range.end);
            RangeMergeResult::Merged(Range::new(start, end))
        } else {
            RangeMergeResult::Unmerged(self, other_range)
        }
    }

    fn overlaps(&self, other_range: &Range) -> bool {
        if self.start <= other_range.start {
            self.end >= other_range.start
        } else {
            other_range.end >= self.start
        }
    }

    fn contains(&self, value: isize) -> bool {
        self.start <= value && value <= self.end
    }
}

fn parse_sensors(text: &str) -> Option<Vec<Sensor>> {
    text.trim().split('\n').map(Sensor::parse).collect()
}

impl Sensor {
    fn parse(text: &str) -> Option<Self> {
        lazy_static! {
            static ref RE: Regex = Regex::new(
                r"^Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)$"
            )
            .unwrap();
        }

        let caps = RE.captures(text)?;
        let parse_isize = |group| caps.get(group).unwrap().as_str().parse().ok();
        let sensor_x = parse_isize(1)?;
        let sensor_y = parse_isize(2)?;
        let beacon_x = parse_isize(3)?;
        let beacon_y = parse_isize(4)?;

        let position = Point {
            x: sensor_x,
            y: sensor_y,
        };
        let closest_beacon = Point {
            x: beacon_x,
            y: beacon_y,
        };

        Some(Sensor {
            position,
            closest_beacon,
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse() {
        let sensors = parse_sensors(include_str!("../../static/example15.txt")).unwrap();
        let expected0 = s(2, 18, -2, 15);
        assert_eq!(expected0, sensors[0]);
        let expected1 = s(9, 16, 10, 16);
        assert_eq!(expected1, sensors[1]);
        let expected2 = s(13, 2, 15, 3);
        assert_eq!(expected2, sensors[2]);
    }

    #[test]
    fn test_merge_ranges() {
        // None should be merged
        let ranges = vec![r(-5, -3), r(-1, 2), r(4, 7)];
        let actual = merge_ranges(ranges);
        let expected = vec![r(-5, -3), r(-1, 2), r(4, 7)];
        assert_eq!(expected, actual);

        // All should be merged
        let ranges = vec![r(-5, -3), r(-4, 2), r(1, 7)];
        let actual = merge_ranges(ranges);
        let expected = vec![r(-5, 7)];
        assert_eq!(expected, actual);

        // Some should be merged
        let ranges = vec![r(-5, -3), r(-4, 2), r(4, 7)];
        let actual = merge_ranges(ranges);
        let expected = vec![r(-5, 2), r(4, 7)];
        assert_eq!(expected, actual);

        // Some fully contain others
        let ranges = vec![r(-5, -3), r(-4, -2), r(4, 7)];
        let actual = merge_ranges(ranges);
        let expected = vec![r(-5, -2), r(4, 7)];
        assert_eq!(expected, actual);

        // Some are "touching" (first end is next start)
        let ranges = vec![r(-5, -3), r(-3, -2), r(4, 7)];
        let actual = merge_ranges(ranges);
        let expected = vec![r(-5, -2), r(4, 7)];
        assert_eq!(expected, actual);

        // Out of order
        let ranges = vec![r(3, 7), r(-2, 4), r(100, 110), r(2, 6)];
        let actual = merge_ranges(ranges);
        let expected = vec![r(-2, 7), r(100, 110)];
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_count_values_in_ranges() {
        let ranges = vec![r(-7, -5), r(-1, 1), r(10, 15)];

        // No values
        let values = vec![];
        assert_eq!(0, count_values_in_ranges(values, ranges.clone()));

        // All before
        let values = vec![-100, -90, -70];
        assert_eq!(0, count_values_in_ranges(values, ranges.clone()));

        // All after
        let values = vec![50, 30, 72];
        assert_eq!(0, count_values_in_ranges(values, ranges.clone()));

        // In between
        let values = vec![4, -3, 9];
        assert_eq!(0, count_values_in_ranges(values, ranges.clone()));

        // All in first
        let values = vec![-7, -5, -6];
        assert_eq!(3, count_values_in_ranges(values, ranges.clone()));

        // All in middle
        let values = vec![1, 0];
        assert_eq!(2, count_values_in_ranges(values, ranges.clone()));

        // All in last
        let values = vec![14, 12, 11, 13, 15];
        assert_eq!(5, count_values_in_ranges(values, ranges.clone()));

        // In each
        let values = vec![12, -5, 1];
        assert_eq!(3, count_values_in_ranges(values, ranges.clone()));

        // Some included, some not
        let values = vec![12, 70, -6, -2, 0];
        assert_eq!(3, count_values_in_ranges(values, ranges.clone()));
    }

    #[test]
    fn test_examples_part1() {
        let sensors = parse_sensors(include_str!("../../static/example15.txt")).unwrap();
        assert_eq!(26, count_eliminated_positions_in_row(&sensors, 10));
    }

    #[test]
    fn test_real_part1() {
        // assert_eq!(0, Day15.part1(Example::Real, Debug::NotDebug));
    }

    #[test]
    fn test_examples_part2() {}

    #[test]
    fn test_real_part2() {
        // assert_eq!(0, Day15.part2(Example::Real, Debug::NotDebug));
    }

    fn s(sensor_x: isize, sensor_y: isize, beacon_x: isize, beacon_y: isize) -> Sensor {
        Sensor {
            position: Point {
                x: sensor_x,
                y: sensor_y,
            },
            closest_beacon: Point {
                x: beacon_x,
                y: beacon_y,
            },
        }
    }

    fn r(start: isize, end: isize) -> Range {
        Range::new(start, end)
    }
}
