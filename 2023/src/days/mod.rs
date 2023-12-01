#![allow(dead_code)]

use std::fs;

pub use day01::Day01;

pub mod day01;

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
    fn run(&self, part: Part, example: Example, debug: Debug) {
        let answer = match part {
            Part::Part1 => self.part1(example, debug),
            Part::Part2 => self.part2(example, debug),
        };
        println!("{}", answer);
    }
    fn part1(&self, example: Example, debug: Debug) -> String;
    fn part2(&self, example: Example, debug: Debug) -> String;
    fn read_file(&self, example: Example) -> String {
        let prefix = match example {
            Example::Real => "input",
            Example::Example => "example",
        };
        let filename = format!("static/{}{:02}.txt", prefix, self.number());
        fs::read_to_string(&filename)
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

impl From<Example> for bool {
    fn from(example: Example) -> Self {
        match example {
            Example::Example => true,
            Example::Real => false,
        }
    }
}

impl From<Debug> for bool {
    fn from(debug: Debug) -> Self {
        match debug {
            Debug::Debug => true,
            Debug::NotDebug => false,
        }
    }
}

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

#[macro_export]
macro_rules! debug_dbg {
    ($($tts:tt)*) => {
        if (DEBUG) {
            dbg!($($tts)*);
        }
    }
}
