extern crate core;

use clap::Parser;

use days::*;

mod days;

const DEFAULT_DAY: u32 = 21;

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
    let part2 = match args.part2 {
        true => Part::Part2,
        false => Part::Part1,
    };
    day.run(part2, args.example.into(), args.debug.into());
}

fn get_day(day_num: u32) -> Option<Box<dyn Day>> {
    Some(match day_num {
        1 => Box::new(Day01),
        2 => Box::new(Day02),
        3 => Box::new(Day03),
        4 => Box::new(Day04),
        5 => Box::new(Day05),
        6 => Box::new(Day06),
        7 => Box::new(Day07),
        8 => Box::new(Day08),
        9 => Box::new(Day09),
        10 => Box::new(Day10),
        11 => Box::new(Day11),
        12 => Box::new(Day12),
        13 => Box::new(Day13),
        14 => Box::new(Day14),
        15 => Box::new(Day15),
        17 => Box::new(Day17),
        18 => Box::new(Day18),
        20 => Box::new(Day20),
        21 => Box::new(Day21),
        _ => return None,
    })
}
