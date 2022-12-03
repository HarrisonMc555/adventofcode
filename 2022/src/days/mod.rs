#![allow(dead_code)]

use std::fs;

pub use day01::Day01;

pub mod day01;

#[derive(Debug, Eq, PartialEq)]
pub enum Part {
    Part1,
    Part2,
}

#[derive(Debug, Eq, PartialEq)]
pub enum Example {
    Real,
    Example,
}

#[derive(Debug, Eq, PartialEq)]
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
        fs::read_to_string(filename).expect("Should have been able to read the file")
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
