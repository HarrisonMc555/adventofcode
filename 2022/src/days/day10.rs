use array2d::Array2D;

use crate::days::{Day, Debug, Example, Part};
use crate::debug_println;

pub struct Day10;

const DEBUG: bool = false;

impl Day for Day10 {
    fn number(&self) -> u32 {
        10
    }

    fn run(&self, part: Part, example: Example, debug: Debug) {
        let answer = match part {
            Part::Part1 => self.part1(example, debug),
            Part::Part2 => self.part2(example, debug),
        };
        println!("{}", answer);
    }
}

impl Day10 {
    fn part1(&self, example: Example, _debug: Debug) -> String {
        let instructions = parse(&self.read_file(example)).unwrap();
        let register_history = run_instructions(&instructions);
        calc_signal_strength(&register_history, &SPECIAL_CYCLES).to_string()
    }

    fn part2(&self, example: Example, _debug: Debug) -> String {
        let instructions = parse(&self.read_file(example)).unwrap();
        let screen = draw_screen(&instructions);
        screen_strings(&screen).join("\n")
    }
}

const SPECIAL_CYCLES: [usize; 6] = [20, 60, 100, 140, 180, 220];

#[derive(Debug, Hash, Clone, Eq, PartialEq)]
enum Instruction {
    Noop,
    Addx(i32),
}

#[derive(Debug, Hash, Default, Copy, Clone, Eq, PartialEq)]
enum Cell {
    #[default]
    Off,
    On,
}

fn run_instructions(instructions: &[Instruction]) -> Vec<i32> {
    let mut register = 1;
    let mut register_history = Vec::new();
    for instruction in instructions {
        for _ in 0..instruction.num_cycles() {
            register_history.push(register);
        }
        instruction.apply_instruction(&mut register);
    }
    register_history.push(register);
    register_history
}

fn calc_signal_strength(register_history: &[i32], special_cycles: &[usize]) -> i32 {
    special_cycles
        .iter()
        .map(|cycle| *cycle as i32 * register_history[*cycle - 1])
        .sum()
}

const SCREEN_WIDTH: usize = 40;
const SCREEN_HEIGHT: usize = 6;

fn draw_screen(instructions: &[Instruction]) -> Array2D<Cell> {
    let register_history = run_instructions(instructions);
    let mut screen = Array2D::filled_with(Cell::Off, SCREEN_HEIGHT, SCREEN_WIDTH);
    for (cycle, register) in register_history.into_iter().enumerate() {
        debug_println!(
            "During cycle    {: >2}: CRT draws in position {}",
            cycle + 1,
            cycle % screen.num_columns()
        );
        debug_println!(
            "Current CRT row   : {}",
            screen
                .row_iter(cycle / screen.num_columns())
                .map(|row| row
                    .take(cycle % screen.num_columns())
                    .map(|cell| cell.to_char())
                    .collect::<String>())
                .unwrap_or_else(|_| "???".to_string())
        );
        debug_println!(
            "Sprite position {: <2}: {}",
            register,
            (0..screen.num_columns())
                .map(|i| {
                    if register - 1 <= i as i32 && i as i32 <= register + 1 {
                        '#'
                    } else {
                        '.'
                    }
                })
                .collect::<String>()
        );
        debug_println!();
        draw_cell(&mut screen, cycle, register);
    }
    screen
}

fn screen_strings(screen: &Array2D<Cell>) -> Vec<String> {
    screen
        .rows_iter()
        .map(|row| row.map(|cell| cell.to_char()).collect())
        .collect()
}

fn draw_cell(screen: &mut Array2D<Cell>, cycle: usize, register: i32) {
    if !sprite_overlaps(screen, cycle, register) {
        return;
    }
    if let Some(cell) = screen.get_mut_row_major(cycle) {
        *cell = Cell::On;
    }
}

fn sprite_overlaps(screen: &Array2D<Cell>, cycle: usize, register: i32) -> bool {
    if register < 0 {
        return false;
    }
    let register = register as usize;
    let min = register.saturating_sub(1);
    let max = register + 1;
    let cycle_index = cycle % screen.num_columns();
    min <= cycle_index && cycle_index <= max
}

impl Instruction {
    fn num_cycles(&self) -> usize {
        match self {
            Instruction::Noop => 1,
            Instruction::Addx(_) => 2,
        }
    }

    fn apply_instruction(&self, register: &mut i32) {
        if let Instruction::Addx(amount) = self {
            *register += amount;
        }
    }
}

fn parse(text: &str) -> Option<Vec<Instruction>> {
    text.trim().split('\n').map(Instruction::parse).collect()
}

impl Instruction {
    fn parse(text: &str) -> Option<Self> {
        if text == "noop" {
            return Some(Instruction::Noop);
        }

        let addx_amount = text.strip_prefix("addx ")?.parse().ok()?;
        Some(Instruction::Addx(addx_amount))
    }
}

impl Cell {
    fn to_char(self) -> char {
        match self {
            Cell::Off => '.',
            Cell::On => '#',
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse() {
        let text = include_str!("../../static/example10.txt");
        let instructions = parse(text);
        assert!(instructions.is_some());

        let text = "noop\n\
                    addx 3\n\
                    addx -5";
        let actual = parse(text).unwrap();
        let expected = vec![
            Instruction::Noop,
            Instruction::Addx(3),
            Instruction::Addx(-5),
        ];
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_examples_part1() {
        let text = "noop\n\
                    addx 3\n\
                    addx -5";
        let instructions = parse(text).unwrap();
        let actual = run_instructions(&instructions);
        let expected = vec![1, 1, 1, 4, 4, -1];
        assert_eq!(expected, actual);

        let text = include_str!("../../static/example10.txt");
        let instructions = parse(text).unwrap();
        let register_history = run_instructions(&instructions);
        let actual = calc_signal_strength(&register_history, &SPECIAL_CYCLES);
        let expected = 13140;
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_real_part1() {
        assert_eq!("17180", Day10.part1(Example::Real, Debug::NotDebug));
    }

    #[test]
    fn test_examples_part2() {
        let text = include_str!("../../static/example10.txt");
        let instructions = parse(text).unwrap();
        let screen = draw_screen(&instructions);
        let actual = screen_strings(&screen).join("\n");
        let expected = "##..##..##..##..##..##..##..##..##..##..\n\
                        ###...###...###...###...###...###...###.\n\
                        ####....####....####....####....####....\n\
                        #####.....#####.....#####.....#####.....\n\
                        ######......######......######......####\n\
                        #######.......#######.......#######.....";
        if expected != actual {
            println!("Expected:\n{}\n\nActual:\n{}\n\n", expected, actual);
        }
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_real_part2() {
        let actual = Day10.part2(Example::Real, Debug::NotDebug);
        let expected = "###..####.#..#.###..###..#....#..#.###..\n\
                        ...#.#....#..#.#..#.#..#.#....#..#.#..#.\n\
                        ...#.###..####.#..#.#..#.#....#..#.###..\n\
                        ###..#....#..#.###..###..#....#..#.#..#.\n\
                        ..#..#....#..#.#....#.#..#....#..#.#..#.\n\
                        ...#.####.#..#.#....#..#.####..##..###..";
        if expected != actual {
            println!("Expected:\n{}\n\nActual:\n{}\n\n", expected, actual);
        }
        assert_eq!(expected, actual);
    }
}
