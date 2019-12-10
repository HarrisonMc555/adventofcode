type Result<T> = std::result::Result<T, ()>;
type Value = u32;

const INPUT: &str = include_str!("../../static/day01.txt");

pub fn main() {
    let answer1 = solve1(INPUT).unwrap();
    let answer2 = solve2(INPUT).unwrap();
    println!("{}", answer1);
    println!("{}", answer2);
}

fn solve1(input: &str) -> Result<Value> {
    let masses = parse(input)?;
    let fuel = masses.into_iter().map(calc_fuel).sum();
    Ok(fuel)
}

fn solve2(input: &str) -> Result<Value> {
    let masses = parse(input)?;
    let total_fuel = masses.into_iter().map(calc_fuel_recursive).sum();
    Ok(total_fuel)
}

fn calc_fuel_recursive(mass: Value) -> Value {
    let new_fuel = calc_fuel(mass);
    if new_fuel == 0 {
        return 0;
    }
    new_fuel + calc_fuel_recursive(new_fuel)
}

fn calc_fuel(mass: Value) -> Value {
    let quotient = mass / 3;
    if quotient < 2 {
        0
    } else {
        quotient - 2
    }
}

fn parse(input: &str) -> Result<Vec<Value>> {
    input.lines().map(|s| s.parse().map_err(|_| ())).collect()
}
