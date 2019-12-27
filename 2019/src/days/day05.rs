use crate::util::intcode::{IntCode, Result, Value};

const INPUT: &str = include_str!("../../static/day05.txt");

lazy_static! {
    static ref PROGRAM_INPUT1: [Value; 1] = [Value::from(1)];
    static ref PROGRAM_INPUT2: [Value; 1] = [Value::from(5)];
}

pub fn main() {
    let answer1 = solve1(INPUT).unwrap();
    println!("{}", answer1);
    let answer2 = solve2(INPUT).unwrap();
    println!("{}", answer2);
}

fn solve1(input: &str) -> Result<Value> {
    let output = IntCode::from_str(input)?
        .with_inputs(PROGRAM_INPUT1.to_vec())
        .run()?
        .last_output()?;
    Ok(output)
}

fn solve2(input: &str) -> Result<Value> {
    let output = IntCode::from_str(input)?
        .with_inputs(PROGRAM_INPUT2.to_vec())
        .run()?
        .last_output()?;
    Ok(output)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn answer1() {
        assert_eq!(solve1(INPUT), Ok(Value::from(6069343)));
    }

    #[test]
    fn answer2() {
        assert_eq!(solve2(INPUT), Ok(Value::from(3188550)));
    }
}
