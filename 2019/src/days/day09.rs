use crate::util::intcode::{IntCode, Result, Value};

const INPUT: &str = include_str!("../../static/day09.txt");

pub fn main() {
    let answer1 = solve1(INPUT);
    println!("{:?}", answer1);
    let answer2 = solve2(INPUT);
    println!("{:?}", answer2);
}

fn solve1(input: &str) -> Result<Value> {
    let outputs = IntCode::from_str(input)?
        .with_inputs(vec![Value::from(1)])
        .run()?
        .outputs()
        .to_vec();
    if outputs.len() == 1 {
        Ok(outputs[0].clone())
    } else {
        let output_strings = outputs
            .iter()
            .map(|v| v.to_string())
            .collect::<Vec<_>>()
            .join(",");
        Err(format!("Invalid opcodes found: {}", output_strings))
    }
}

fn solve2(input: &str) -> Result<Value> {
    Ok(IntCode::from_str(input)?
        .with_inputs(vec![Value::from(2)])
        .run()?
        .first_output()?)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn answer1() {
        assert_eq!(solve1(INPUT), Ok("3235019597".parse().unwrap()));
    }

    #[test]
    fn answer2() {
        assert_eq!(solve2(INPUT), Ok(Value::from(80274)));
    }
}
