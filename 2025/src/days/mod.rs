#![allow(dead_code)]

use std::fs;

pub use day01::Day01;
pub use day02::Day02;
pub use day03::Day03;
pub use day04::Day04;
pub use day05::Day05;
pub use day06::Day06;
pub use day07::Day07;
pub use day08::Day08;
pub use day09::Day09;

pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
pub mod day05;
pub mod day06;
pub mod day07;
pub mod day08;
pub mod day09;

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
    fn read_file(&self, part: Part, example: Example) -> String {
        let prefix = match example {
            Example::Real => "input",
            Example::Example => "example",
        };
        let suffix = match part {
            Part::Part1 => "1",
            Part::Part2 => "2",
        };
        let filename_with_suffix = format!("static/{prefix}{:02}_part{suffix}.txt", self.number());
        if let Ok(contents) = fs::read_to_string(filename_with_suffix) {
            return contents;
        }
        let filename = format!("static/{}{:02}.txt", prefix, self.number());
        fs::read_to_string(&filename)
            .unwrap_or_else(|e| panic!("Should have been able to read file {}: {:?}", filename, e))
    }
    fn get_lines(&self, part: Part, example: Example) -> Vec<String> {
        self.read_file(part, example)
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
    ($debug:expr) => {
        if $debug.into() {
            use std::io::Write;
            println!();
            std::io::stdout().flush().expect("Could not flush stdout");
        }
    };
    ($debug:expr, $($tts:tt)*) => {
        if $debug.into() {
            use std::io::Write;
            println!($($tts)*);
            std::io::stdout().flush().expect("Could not flush stdout");
        }
    };
}

#[macro_export]
macro_rules! debug_print {
    ($debug:expr) => {
        if $debug.into() {
            use std::io::Write;
            print!();
            std::io::stdout().flush().expect("Could not flush stdout");
        }
    };
    ($debug:expr, $($tts:tt)*) => {
        if $debug.into() {
            use std::io::Write;
            print!($($tts)*);
            std::io::stdout().flush().expect("Could not flush stdout");
        }
    }
}

#[macro_export]
macro_rules! debug_dbg {
    ($debug:expr) => {
        if $debug.into() {
            use std::io::Write;
            dbg!();
            std::io::stdout().flush().expect("Could not flush stdout");
        }
    };
    ($debug:expr, $($tts:tt)*) => {
        if $debug.into() {
            use std::io::Write;
            dbg!($($tts)*);
            std::io::stdout().flush().expect("Could not flush stdout");
        }
    }
}
