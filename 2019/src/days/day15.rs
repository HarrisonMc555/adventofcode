use crate::util::intcode::{Error, IntCode, Result, Stopped, Value};

const INPUT: &str = include_str!("../../static/day15.txt");

pub fn main() {
    let answer1 = solve1(INPUT);
    println!("{:?}", answer1);
    // let answer2 = solve2(INPUT);
    // println!("{:?}", answer2);
}

fn solve1(input: &str) -> Result<()> {
    Err("unimplemented".to_string())
}
