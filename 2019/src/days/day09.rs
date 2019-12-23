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
    fn test_example1() -> Result<()> {
        let program = "109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99";
        let product = IntCode::from_str(program)?.run()?;
        let memory = product.memory();
        let memory_as_str = (0..16)
            .map(|i| memory.get(&i).cloned().unwrap_or_else(|| Value::from(0)))
            .map(|v| v.to_string())
            .collect::<Vec<_>>()
            .join(",");
        assert_eq!(memory_as_str, program);
        Ok(())
    }

    #[test]
    fn test_example2() -> Result<()> {
        let program = "1102,34915192,34915192,7,4,7,99,0";
        let output = IntCode::from_str(program)?.run()?.first_output()?;
        let num_digits = output.to_string().chars().count();
        assert_eq!(num_digits, 16);
        Ok(())
    }

    #[test]
    fn test_example3() -> Result<()> {
        let program = "104,1125899906842624,99";
        let output = IntCode::from_str(program)?.run()?.first_output()?;
        assert_eq!(
            output,
            "1125899906842624".parse().map_err(|_| "Cannot parse")?
        );
        Ok(())
    }

    #[test]
    fn answer1() {
        assert_eq!(solve1(INPUT), Ok("3235019597".parse().unwrap()));
    }

    #[test]
    fn answer2() {
        assert_eq!(solve2(INPUT), Ok(Value::from(80274)));
    }
}
