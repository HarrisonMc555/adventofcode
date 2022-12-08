use crate::days::{Day, Debug, Example, Part};
use array2d::Array2D;

pub struct Day08;

impl Day for Day08 {
    fn number(&self) -> u32 {
        8
    }

    fn run(&self, part: Part, example: Example, debug: Debug) {
        let answer = match part {
            Part::Part1 => self.part1(example, debug),
            Part::Part2 => self.part2(example, debug),
        };
        println!("{}", answer);
    }
}

impl Day08 {
    fn part1(&self, example: Example, _debug: Debug) -> usize {
        let text = self.read_file(example);
        let trees = parse_trees(&text).unwrap();
        count_visible(&trees)
    }

    fn part2(&self, _example: Example, _debug: Debug) -> usize {
        todo!()
    }
}

const BASE: u32 = 10;

fn count_visible(trees: &Array2D<u32>) -> usize {
    let mut count = 0;
    for row in 0..trees.num_rows() {
        for column in 0..trees.num_columns() {
            if is_visible(trees, row, column) {
                count += 1;
            }
        }
    }
    count
}

fn is_visible(trees: &Array2D<u32>, row: usize, column: usize) -> bool {
    let tree = trees[(row, column)];

    let above = (0..row).map(|r| (r, column));
    let below = ((row + 1)..trees.num_rows()).map(|r| (r, column));
    let left = (0..column).map(|c| (row, c));
    let right = ((column + 1)..trees.num_columns()).map(|c| (row, c));

    let to_tree = |(r, c)| trees[(r, c)];
    let lower = |t| t < tree;

    above.map(to_tree).all(lower)
        || below.map(to_tree).all(lower)
        || left.map(to_tree).all(lower)
        || right.map(to_tree).all(lower)
}

fn parse_trees(text: &str) -> Option<Array2D<u32>> {
    let grid = text
        .trim()
        .split('\n')
        .map(parse_line)
        .collect::<Option<Vec<_>>>()?;
    Some(Array2D::from_rows(&grid))
}

fn parse_line(line: &str) -> Option<Vec<u32>> {
    line.chars().map(|c| c.to_digit(BASE)).collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_examples_part1() {
        let text = include_str!("../../static/example08.txt");
        let trees = parse_trees(text).unwrap();
        let visible = count_visible(&trees);
        assert_eq!(21, visible);
    }

    #[test]
    fn test_examples_part2() {}
}
