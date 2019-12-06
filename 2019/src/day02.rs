use std::ops;

type Result<T> = std::result::Result<T, ()>;
type Value = usize;

const INPUT: &str = include_str!("../static/day02.txt");

const OPCODE_ADD: Value = 1;
const OPCODE_MUL: Value = 2;
const OPCODE_HALT: Value = 99;

const MAX_NOUN: Value = 99;
const MAX_VERB: Value = 99;

const DEFAULT_NOUN: Value = 12;
const DEFAULT_VERB: Value = 2;

const OUTPUT_GOAL: Value = 19690720;

type Op = fn(Value, Value) -> Value;

pub fn main() {
    let answer1 = solve1(INPUT, DEFAULT_NOUN, DEFAULT_VERB).unwrap();
    let answer2 = solve2(INPUT, OUTPUT_GOAL).unwrap();
    println!("{}", answer1);
    println!("{}", answer2);
}

fn solve1(input: &str, noun: Value, verb: Value) -> Result<Value> {
    let program = parse_input(input)?;
    run_altered(program, noun, verb)
}

fn solve2(input: &str, output_goal: Value) -> Result<Value> {
    let program = parse_input(input)?;
    for noun in 0..=MAX_NOUN {
        for verb in 0..=MAX_VERB {
            if run_altered(program.clone(), noun, verb) == Ok(output_goal) {
                return Ok(100 * noun + verb);
            }
        }
    }
    Err(())
}

fn run_altered(mut program: Vec<Value>, noun: Value, verb: Value) -> Result<Value> {
    alter_program(&mut program, noun, verb)?;
    run(program)
}

fn run(mut program: Vec<Value>) -> Result<Value> {
    let mut index = 0;
    while let Some(new_index) = step(&mut program, index)? {
        index = new_index;
    }
    let answer = get_copy_or_err(&program, 0)?;
    Ok(answer)
}

fn step(program: &mut [Value], index: usize) -> Result<Option<usize>> {
    let opcode = get_copy_or_err(program, index)?;
    let op: Op = match opcode {
        OPCODE_ADD => ops::Add::add,
        OPCODE_MUL => ops::Mul::mul,
        OPCODE_HALT => return Ok(None),
        _ => return Err(()),
    };
    let op1_index = get_copy_or_err(program, index + 1)?;
    let op2_index = get_copy_or_err(program, index + 2)?;
    let dest_index = get_copy_or_err(program, index + 3)?;
    let op1 = get_copy_or_err(program, op1_index)?;
    let op2 = get_copy_or_err(program, op2_index)?;
    let dest = get_mut_or_err(program, dest_index)?;
    *dest = op(op1, op2);
    Ok(Some(index + 4))
}

fn alter_program(program: &mut [Value], noun: Value, verb: Value) -> Result<()> {
    *get_mut_or_err(program, 1)? = noun;
    *get_mut_or_err(program, 2)? = verb;
    Ok(())
}

fn get_copy_or_err<T: Copy>(arr: &[T], index: usize) -> Result<T> {
    arr.get(index).ok_or(()).map(std::clone::Clone::clone)
}

fn get_mut_or_err<T>(arr: &mut [T], index: usize) -> Result<&mut T> {
    arr.get_mut(index).ok_or(())
}

fn parse_input(input: &str) -> Result<Vec<Value>> {
    input
        .trim()
        .split(',')
        .map(str::parse)
        .collect::<std::result::Result<_, _>>()
        .map_err(|_| ())
}
