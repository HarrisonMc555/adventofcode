use array2d::Array2D;

use crate::days::{Day, Debug, Example, Part};

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

    fn part2(&self, example: Example, _debug: Debug) -> usize {
        let text = self.read_file(example);
        let trees = parse_trees(&text).unwrap();
        find_best_score(&trees)
    }
}

const BASE: u32 = 10;

fn count_visible(trees: &Array2D<u32>) -> usize {
    (0..trees.num_rows())
        .flat_map(|r| (0..trees.num_columns()).map(move |c| (r, c)))
        .filter(|(r, c)| is_visible(trees, *r, *c))
        .count()
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

fn find_best_score(trees: &Array2D<u32>) -> usize {
    (0..trees.num_rows())
        .flat_map(|r| (0..trees.num_columns()).map(move |c| (r, c)))
        .map(|(r, c)| score(trees, r, c))
        .max()
        .unwrap_or(0)
}

fn score(trees: &Array2D<u32>, row: usize, column: usize) -> usize {
    let tree = trees[(row, column)];
    let to_tree = |(r, c)| trees[(r, c)];

    let above = (0..row).map(|r| (r, column)).rev();
    let below = ((row + 1)..trees.num_rows()).map(|r| (r, column));
    let left = (0..column).map(|c| (row, c)).rev();
    let right = ((column + 1)..trees.num_columns()).map(|c| (row, c));

    score_direction(above.map(to_tree), tree)
        * score_direction(below.map(to_tree), tree)
        * score_direction(left.map(to_tree), tree)
        * score_direction(right.map(to_tree), tree)
}

fn score_direction<T>(iter: T, tree: u32) -> usize
where
    T: Iterator<Item = u32>,
{
    let mut count = 0;
    for t in iter {
        count += 1;
        if t >= tree {
            break;
        }
    }
    count
}

fn parse_trees(text: &str) -> Option<Array2D<u32>> {
    let grid = text
        .trim()
        .split('\n')
        .map(parse_line)
        .collect::<Option<Vec<_>>>()?;
    Array2D::from_rows(&grid).ok()
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
    fn test_examples_part2() {
        let text = include_str!("../../static/example08.txt");
        let trees = parse_trees(text).unwrap();
        let best_score = find_best_score(&trees);
        assert_eq!(8, best_score);
    }
}
