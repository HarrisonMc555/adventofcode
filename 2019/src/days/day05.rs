use crate::util::intcode::{IntCode, Result, Value};

const INPUT: &str = include_str!("../../static/day05.txt");
const PROGRAM_INPUT: [Value; 1] = [1];

pub fn main() {
    let answer1 = solve1(INPUT).unwrap();
    println!("{}", answer1);
}

fn solve1(input: &str) -> Result<Value> {
    IntCode::from_str(input)?
        .with_inputs(PROGRAM_INPUT.to_vec())
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
