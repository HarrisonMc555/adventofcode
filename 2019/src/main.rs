#[macro_use]
extern crate lazy_static;

use std::env;

mod days;
mod util;

use days::*;

const DEFAULT_FN: fn() = day17::main;

fn main() {
    let f = env::args()
        .nth(1)
        .and_then(|num| {
            Some(match num.as_ref() {
                "1" => day01::main,
                "2" => day02::main,
                "3" => day03::main,
                "4" => day04::main,
                "5" => day05::main,
                "6" => day06::main,
                "7" => day07::main,
                "8" => day08::main,
                "9" => day09::main,
                "10" => day10::main,
                "11" => day11::main,
                "12" => day12::main,
                "13" => day13::main,
                "14" => day14::main,
                "15" => day15::main,
                "16" => day16::main,
                "17" => day17::main,
                _ => {
                    eprintln!("Unimplmented day {}", num);
                    return None;
                }
            })
        })
        .unwrap_or(DEFAULT_FN);
    f();
}
