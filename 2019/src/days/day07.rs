use crate::util::intcode::{IntCode, Result, Stopped, Value};
use std::collections::VecDeque;

const INPUT: &str = include_str!("../../static/day07.txt");

const NUM_AMPLIFIERS: usize = 5;
lazy_static! {
    static ref STARTING_INPUT_SIGNAL: Value = Value::from(0);
}

pub fn main() {
    let answer1 = solve1(INPUT);
    println!("{:?}", answer1);
    let answer2 = solve2(INPUT);
    println!("{:?}", answer2);
}

fn solve1(input: &str) -> Result<Value> {
    let program = IntCode::from_str(input)?;
    let all_phase_settings = (0..NUM_AMPLIFIERS).map(Value::from).collect::<Vec<_>>();
    permutations(&all_phase_settings)
        .into_iter()
        .flat_map(|phase_settings| run_amplifiers(program.clone(), &phase_settings).ok())
        .max()
        .ok_or("No successful runs".to_string())
}

fn solve2(input: &str) -> Result<Value> {
    let program = IntCode::from_str(input)?;
    let all_phase_settings = (5..=9).map(Value::from).collect::<Vec<_>>();
    permutations(&all_phase_settings)
        .into_iter()
        .flat_map(|phase_settings| run_amplifiers_feedback(program.clone(), &phase_settings).ok())
        .max()
        .ok_or("No successful runs".to_string())
}

fn run_amplifiers(program: IntCode, phase_settings: &[Value]) -> Result<Value> {
    let mut input_signal = (*STARTING_INPUT_SIGNAL).clone();
    for phase_setting in phase_settings {
        input_signal = run_amplifier(program.clone(), phase_setting.clone(), input_signal)?;
    }
    Ok(input_signal)
}

fn run_amplifier(program: IntCode, phase_setting: Value, input_signal: Value) -> Result<Value> {
    let output = program
        .with_inputs(vec![phase_setting, input_signal])
        .run()?
        .last_output()?;
    Ok(output.clone())
}

fn run_amplifiers_feedback(program: IntCode, phase_settings: &[Value]) -> Result<Value> {
    let mut programs = phase_settings
        .iter()
        .enumerate()
        .map(|(i, phase_setting)| (i, program.clone().with_inputs(vec![phase_setting.clone()])))
        .collect::<VecDeque<_>>();
    let mut input_signal = (*STARTING_INPUT_SIGNAL).clone();
    let last_amplifier_index = programs.len() - 1;
    let mut recent_last_amplifier_output: Option<Value> = None;
    loop {
        let (i, program) = match programs.pop_front() {
            Some(some) => some,
            None => {
                return recent_last_amplifier_output
                    .ok_or_else(|| "Last amplifier never produced output".to_string())
            }
        };
        let output_signal = match run_amplifier_til_input(program, input_signal)? {
            Stopped::NeedInput(mut program) => {
                let output_signal = program.pop_output();
                programs.push_back((i, program));
                output_signal?
            }
            Stopped::Complete(product) => {
                // Don't push back on stack, it's done
                product.last_output()?.clone()
            }
        };
        if i == last_amplifier_index {
            recent_last_amplifier_output = Some(output_signal.clone());
        }
        input_signal = output_signal;
    }
}

fn run_amplifier_til_input(mut program: IntCode, input_signal: Value) -> Result<Stopped> {
    program.push_input(&input_signal);
    program.run_blocking_input()
}

fn permutations<T>(arr: &[T]) -> Vec<Vec<T>>
where
    T: Clone,
{
    match arr.len() {
        0 => Vec::new(),
        1 => vec![arr.to_vec()],
        _ => {
            let value = &arr[0];
            permutations(&arr[1..])
                .iter()
                .flat_map(|rest| {
                    (0..arr.len())
                        .map(|i| {
                            let mut rest_clone = rest.clone();
                            rest_clone.insert(i, value.clone());
                            rest_clone
                        })
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>()
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn max_signal1() -> Result<()> {
        let input = "3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0";
        let phase_settings = value_vec(&[4, 3, 2, 1, 0]);
        let output_signal = Value::from(43210);
        test_amplifiers(input, &phase_settings, output_signal)
    }

    #[test]
    fn max_signal2() -> Result<()> {
        let input = "3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0";
        let phase_settings = value_vec(&[0, 1, 2, 3, 4]);
        let output_signal = Value::from(54321);
        test_amplifiers(input, &phase_settings, output_signal)
    }

    #[test]
    fn max_signal3() -> Result<()> {
        let input = "3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0";
        let phase_settings = value_vec(&[1, 0, 4, 3, 2]);
        let output_signal = Value::from(65210);
        test_amplifiers(input, &phase_settings, output_signal)
    }

    fn test_amplifiers(
        input: &str,
        phase_settings: &[Value],
        expected_signal: Value,
    ) -> Result<()> {
        let program = IntCode::from_str(input)?;
        let actual_signal = run_amplifiers(program, &phase_settings)?;
        assert_eq!(actual_signal, expected_signal);
        Ok(())
    }

    #[test]
    fn max_signal_feedback1() -> Result<()> {
        let input =
            "3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5";
        let phase_settings = value_vec(&[9, 8, 7, 6, 5]);
        let output_signal = Value::from(139629729);
        test_amplifiers_feedback(input, &phase_settings, output_signal)
    }

    #[test]
    fn max_signal_feedback2() -> Result<()> {
        let input = "3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,-5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10";
        let phase_settings = value_vec(&[9, 7, 8, 5, 6]);
        let output_signal = Value::from(18216);
        test_amplifiers_feedback(input, &phase_settings, output_signal)
    }

    fn test_amplifiers_feedback(
        input: &str,
        phase_settings: &[Value],
        expected_signal: Value,
    ) -> Result<()> {
        let program = IntCode::from_str(input)?;
        let actual_signal = run_amplifiers_feedback(program, &phase_settings)?;
        assert_eq!(actual_signal, expected_signal);
        Ok(())
    }

    #[test]
    fn test_permutations1() {
        assert_eq!(permutations(&[1]), vec![vec![1]]);
    }

    #[test]
    fn test_permutations2() {
        let mut expected = vec![vec![1, 2], vec![2, 1]];
        expected.sort();
        let mut actual = permutations(&[1, 2]);
        actual.sort();
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_permutations3() {
        let mut expected = vec![
            vec![1, 2, 3],
            vec![1, 3, 2],
            vec![2, 1, 3],
            vec![2, 3, 1],
            vec![3, 1, 2],
            vec![3, 2, 1],
        ];
        expected.sort();
        let mut actual = permutations(&[1, 2, 3]);
        actual.sort();
        assert_eq!(actual, expected);
    }

    fn value_vec(values: &[u32]) -> Vec<Value> {
        values.iter().map(|&x| Value::from(x)).collect()
    }

    #[test]
    fn answer1() {
        assert_eq!(solve1(INPUT), Ok(Value::from(38500)));
    }

    #[test]
    fn answer2() {
        assert_eq!(solve2(INPUT), Ok(Value::from(33660560)));
    }
}
