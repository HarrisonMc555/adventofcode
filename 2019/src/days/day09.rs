use crate::util::intcode::{Result, Value};

const INPUT: &str = include_str!("../../static/day09.txt");

pub fn main() {
    let answer1 = solve1(INPUT);
    println!("{:?}", answer1);
    let answer2 = solve2(INPUT);
    println!("{:?}", answer2);
}

fn solve1(_input: &str) -> Result<Value> {
    Err(String::new())
}

fn solve2(_input: &str) -> Result<Value> {
    Err(String::new())
}
