use crate::util::intcode::{IntCode, Result, Value};
use num_iter::range_inclusive;
use num_traits::identities::Zero;

const INPUT: &str = include_str!("../../static/day02.txt");

lazy_static! {
    static ref MAX_NOUN: Value = Value::from(99);
    static ref MAX_VERB: Value = Value::from(99);
    static ref DEFAULT_NOUN: Value = Value::from(12);
    static ref DEFAULT_VERB: Value = Value::from(2);
    static ref OUTPUT_GOAL: Value = Value::from(19690720);
}

pub fn main() {
    let answer1 = solve1(INPUT, &*DEFAULT_NOUN, &*DEFAULT_VERB).unwrap();
    println!("{}", answer1);
    let answer2 = solve2(INPUT, &*OUTPUT_GOAL).unwrap();
    println!("{}", answer2);
}

fn solve1(input: &str, noun: &Value, verb: &Value) -> Result<Value> {
    let output = IntCode::from_str(input)?
        .altered(noun, verb)?
        .run()?
        .get_memory_at(0)?
        .clone();
    Ok(output)
}

fn solve2(input: &str, output_goal: &Value) -> Result<Value> {
    let program = IntCode::from_str(input)?;
    for noun in range_inclusive(Value::zero(), MAX_NOUN.clone()) {
        for verb in range_inclusive(Value::zero(), MAX_VERB.clone()) {
            let result = program
                .clone()
                .altered(&noun, &verb)?
                .run()
                .and_then(|product| product.get_memory_at(0));
            if result.as_ref() == Ok(output_goal) {
                return Ok(100 * noun + verb);
            }
        }
    }
    Err("No solution found for solve2".to_string())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn answer02a() {
        assert_eq!(
            solve1(INPUT, &*DEFAULT_NOUN, &*DEFAULT_VERB),
            Ok(Value::from(6568671))
        );
    }

    #[test]
    fn answer02b() {
        assert_eq!(solve2(INPUT, &*OUTPUT_GOAL), Ok(Value::from(3951)));
    }
}
