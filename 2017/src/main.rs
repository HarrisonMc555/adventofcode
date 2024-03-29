extern crate core;

mod days;

use std::ops::Not;

use clap::Parser;
use days::*;

const DEFAULT_DAY: u32 = 2;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = DEFAULT_DAY)]
    day: u32,

    #[arg(short, long)]
    part2: bool,

    #[arg(short, long)]
    example: bool,

    #[arg(long)]
    debug: bool,
}

fn main() {
    let args = Args::parse();
    if args.debug {
        println!("Args: {:?}", args);
    }
    let Some(day) = get_day(args.day) else {
        eprintln!("Invalid day: {}", args.day);
        return;
    };
    day.run(
        args.part2.not().into(),
        args.example.into(),
        args.debug.into(),
    );
}

fn get_day(day_num: u32) -> Option<Box<dyn Day>> {
    Some(match day_num {
        1 => Box::new(Day01),
        2 => Box::new(Day02),
        _ => return None,
    })
}
