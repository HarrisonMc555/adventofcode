use std::env;

mod intcode;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;

const DEFAULT_FN: fn() = day05::main;

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
                _ => {
                    eprintln!("Unimplmented day {}", num);
                    return None;
                }
            })
        })
        .unwrap_or(DEFAULT_FN);
    f();
}
