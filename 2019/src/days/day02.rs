use crate::util::intcode::{IntCode, Result, Value, MAX_NOUN, MAX_VERB};

const INPUT: &str = include_str!("../../static/day02.txt");

const DEFAULT_NOUN: Value = 12;
const DEFAULT_VERB: Value = 2;

const OUTPUT_GOAL: Value = 19690720;

pub fn main() {
    let answer1 = solve1(INPUT, DEFAULT_NOUN, DEFAULT_VERB).unwrap();
    let answer2 = solve2(INPUT, OUTPUT_GOAL).unwrap();
    println!("{}", answer1);
    println!("{}", answer2);
}

fn solve1(input: &str, noun: Value, verb: Value) -> Result<Value> {
    // let program = parse_input(input)?;
    // run_altered(program, noun, verb)
    IntCode::from_str(input)?.altered(noun, verb)?.run()?.get(0)
}

fn solve2(input: &str, output_goal: Value) -> Result<Value> {
    let program = IntCode::from_str(input)?;
    for noun in 0..=MAX_NOUN {
        for verb in 0..=MAX_VERB {
            if program.clone().altered(noun, verb)?.run()?.get(0) == Ok(output_goal) {
                return Ok(100 * noun + verb);
            }
        }
    }
    Err(())
}
