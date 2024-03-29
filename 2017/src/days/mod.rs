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

pub use day01::Day01;
pub use day02::Day02;

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
