use std::env;

mod day01;
mod day02;

const DEFAULT_FN: fn() = day02::main;

fn main() {
    let f = env::args()
        .nth(1)
        .and_then(|num| {
            Some(match num.as_ref() {
                "1" => day01::main,
                "2" => day02::main,
                _ => {
                    eprintln!("Unimplmented day {}", num);
                    return None;
                }
            })
        })
        .unwrap_or(DEFAULT_FN);
    f();
}
