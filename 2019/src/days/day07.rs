use crate::util::intcode::{IntCode, Result, Value};

const INPUT: &str = include_str!("../../static/day07.txt");
const NUM_AMPLIFIERS: Value = 5;
const STARTING_INPUT_SIGNAL: Value = 0;

pub fn main() {
    let answer1 = solve1(INPUT);
    // let answer2 = solve2(INPUT).unwrap();
    println!("{:?}", answer1);
    // println!("{}", answer2);
}

fn solve1(input: &str) -> Result<Value> {
    let program = IntCode::from_str(input)?;
    let all_phase_settings = (0..NUM_AMPLIFIERS).collect::<Vec<_>>();
    permutations(&all_phase_settings)
        .into_iter()
        .flat_map(|phase_settings| run_amplifiers(program.clone(), &phase_settings).ok())
        .max()
        .ok_or("No successful runs".to_string())
}

fn run_amplifiers(program: IntCode, phase_settings: &[Value]) -> Result<Value> {
    let mut input_signal = STARTING_INPUT_SIGNAL;
    for &phase_setting in phase_settings {
        input_signal = run_amplifier(program.clone(), phase_setting, input_signal)?;
    }
    Ok(input_signal)
}

fn run_amplifier(program: IntCode, phase_setting: Value, input_signal: Value) -> Result<Value> {
    program
        .with_inputs(vec![phase_setting, input_signal])
        .run()?
        .last_output()
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
        let phase_settings = [4, 3, 2, 1, 0];
        let output_signal = 43210;
        test_amplifiers(input, &phase_settings, output_signal)
    }

    #[test]
    fn max_signal2() -> Result<()> {
        let input = "3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0";
        let phase_settings = [0, 1, 2, 3, 4];
        let output_signal = 54321;
        test_amplifiers(input, &phase_settings, output_signal)
    }

    #[test]
    fn max_signal3() -> Result<()> {
        let input = "3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0";
        let phase_settings = [1, 0, 4, 3, 2];
        let output_signal = 65210;
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

    #[test]
    fn answer01a() {
        assert_eq!(solve1(INPUT), Ok(38500));
    }

    // #[test]
    // fn answer01b() {
    //     assert_eq!(solve2(INPUT), Ok(3188550));
    // }
}
