use std::env;

mod util;
mod days;

use days::*;

const DEFAULT_FN: fn() = day07::main;

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
                _ => {
                    eprintln!("Unimplmented day {}", num);
                    return None;
                }
            })
        })
        .unwrap_or(DEFAULT_FN);
    f();
}
