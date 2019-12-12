use crate::util::intcode::{IntCode, Result, Value};

// const INPUT: &str = include_str!("../../static/day05.txt");
const INPUT: &str = "3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99";
const PROGRAM_INPUT1: [Value; 1] = [1];
const PROGRAM_INPUT2: [Value; 1] = [5];

pub fn main() {
    // let answer1 = solve1(INPUT).unwrap();
    let answer2 = solve2(INPUT);
    // println!("{}", answer1);
    println!("{:?}", answer2);
}

fn solve1(input: &str) -> Result<Value> {
    IntCode::from_str(input)?
        .with_inputs(PROGRAM_INPUT1.to_vec())
        .run()?
        .outputs()
        .first()
        .copied()
        .ok_or("No outputs".to_string())
}

fn solve2(input: &str) -> Result<Value> {
    IntCode::from_str(input)?
        .with_inputs(PROGRAM_INPUT2.to_vec())
        .run()?
        .outputs()
        .first()
        .copied()
        .ok_or("No outputs".to_string())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn input_output() -> Result<()> {
        let input = 78;
        let output = IntCode::from_str("3,0,4,0,99")?
            .with_inputs(vec![input])
            .run()?
            .outputs()
            .last()
            .copied()
            .ok_or("No outputs".to_string());
        assert_eq!(output, Ok(input));
        Ok(())
    }

    #[test]
    fn immediate() -> Result<()> {
        let product = IntCode::from_str("1002,4,3,4,33")?.run()?;
        assert_eq!(product.memory(), &[1002, 4, 3, 4, 99]);
        Ok(())
    }
}
