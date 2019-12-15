use crate::util::intcode::{IntCode, Product, Result, Value, MAX_NOUN, MAX_VERB};

// const INPUT: &str = "1,1,1,4,99,5,6,0,99";
const INPUT: &str = include_str!("../../static/day02.txt");

const DEFAULT_NOUN: Value = 12;
const DEFAULT_VERB: Value = 2;

const OUTPUT_GOAL: Value = 19690720;

pub fn main() {
    let answer1 = solve1(INPUT, DEFAULT_NOUN, DEFAULT_VERB).unwrap();
    println!("{}", answer1);
    let answer2 = solve2(INPUT, OUTPUT_GOAL).unwrap();
    println!("{}", answer2);
}

fn solve1(input: &str, noun: Value, verb: Value) -> Result<Value> {
    IntCode::from_str(input)?.altered(noun, verb)?.run()?.get_output(0)
}

fn solve2(input: &str, output_goal: Value) -> Result<Value> {
    let program = IntCode::from_str(input)?;
    for noun in 0..=MAX_NOUN {
        for verb in 0..=MAX_VERB {
            let result: Result<Product> = program.clone().altered(noun, verb)?.run();
            if result.and_then(|product| product.get_output(0)) == Ok(output_goal) {
                return Ok(100 * noun + verb);
            }
        }
    }
    // Err(())
    Err("No solution found for solve2".to_string())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn answer02a() {
        assert_eq!(solve1(INPUT, DEFAULT_NOUN, DEFAULT_VERB), Ok(6568671));
    }

    #[test]
    fn answer02b() {
        assert_eq!(solve2(INPUT, OUTPUT_GOAL), Ok(3951));
    }
}
