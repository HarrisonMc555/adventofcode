type Result<T> = std::result::Result<T, ()>;

const INPUT: &str = include_str!("../static/day01.txt");

pub fn main() {
    let answer = solve(INPUT).unwrap();
    let answer2 = solve2(INPUT).unwrap();
    println!("{}", answer);
    println!("{}", answer2);
}

fn solve2(input: &str) -> Result<u64> {
    let masses = parse(input)?;
    let total_fuel = masses.into_iter().map(calc_fuel_recursive).sum();
    Ok(total_fuel)
}

fn solve(input: &str) -> Result<u64> {
    let masses = parse(input)?;
    let answer = masses.into_iter().map(calc_fuel).sum();
    Ok(answer)
}

fn calc_fuel_recursive(mass: u64) -> u64 {
    let new_fuel = calc_fuel(mass);
    if new_fuel == 0 {
        return 0;
    }
    new_fuel + calc_fuel_recursive(new_fuel)
}

fn calc_fuel(mass: u64) -> u64 {
    let quotient = mass / 3;
    if quotient < 2 {
        0
    } else {
        quotient - 2
    }
}

fn parse(input: &str) -> Result<Vec<u64>> {
    input.lines().map(|s| s.parse().map_err(|_| ())).collect()
}
