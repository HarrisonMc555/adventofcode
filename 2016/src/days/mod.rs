#![allow(dead_code)]

#[macro_export]
macro_rules! debug_println {
    ($($tts:tt)*) => {
        if (DEBUG) {
            println!($($tts)*);
        }
    }
}
#[macro_export]
macro_rules! debug_print {
    ($($tts:tt)*) => {
        if (DEBUG) {
            print!($($tts)*);
        }
    }
}

use std::fs;

pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
pub mod day05;
pub mod day06;
pub mod day07;
pub mod day08;
pub mod day09;
pub mod day10;
pub mod day12;
pub mod day13;
pub mod day14;
pub mod day15;
pub mod day16;
pub mod day17;
pub mod day18;
pub mod day19;
pub mod day20;
pub mod day21;
pub mod day22;

pub use day01::Day01;
pub use day02::Day02;
pub use day03::Day03;
pub use day04::Day04;
pub use day05::Day05;
pub use day06::Day06;
pub use day07::Day07;
pub use day08::Day08;
pub use day09::Day09;
pub use day10::Day10;
pub use day12::Day12;
pub use day13::Day13;
pub use day14::Day14;
pub use day15::Day15;
pub use day16::Day16;
pub use day17::Day17;
pub use day18::Day18;
pub use day19::Day19;
pub use day20::Day20;
pub use day21::Day21;
pub use day22::Day22;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Part {
    Part1,
    Part2,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Example {
    Real,
    Example,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Debug {
    NotDebug,
    Debug,
}

pub trait Day {
    fn number(&self) -> u32;
    fn run(&self, part: Part, example: Example, debug: Debug);
    fn read_file(&self, example: Example) -> String {
        let prefix = match example {
            Example::Real => "input",
            Example::Example => "example",
        };
        let filename = format!("static/{}{:02}.txt", prefix, self.number());
        fs::read_to_string(&filename)
            .unwrap_or_else(|e| panic!("Should have been able to read file {}: {:?}", filename, e))
    }
    fn read_bytes(&self, example: Example) -> Vec<u8> {
        let prefix = match example {
            Example::Real => "input",
            Example::Example => "example",
        };
        let filename = format!("static/{}{:02}.txt", prefix, self.number());
        fs::read(&filename)
            .unwrap_or_else(|e| panic!("Should have been able to read file {}: {:?}", filename, e))
    }
    fn get_lines(&self, example: Example) -> Vec<String> {
        self.read_file(example)
            .trim()
            .lines()
            .map(str::to_owned)
            .collect()
    }
}

impl From<bool> for Part {
    fn from(is_part1: bool) -> Self {
        if is_part1 {
            Self::Part1
        } else {
            Self::Part2
        }
    }
}

impl From<bool> for Example {
    fn from(is_example: bool) -> Self {
        if is_example {
            Self::Example
        } else {
            Self::Real
        }
    }
}

impl From<bool> for Debug {
    fn from(is_debug: bool) -> Self {
        if is_debug {
            Self::Debug
        } else {
            Self::NotDebug
        }
    }
}
