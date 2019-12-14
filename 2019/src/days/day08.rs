use array2d::Array2D;

const INPUT: &str = include_str!("../../static/day08.txt");

const BASE: u32 = 10;
const DEFAULT_WIDTH: usize = 25;
const DEFAULT_HEIGHT: usize = 6;

type Result<T> = std::result::Result<T, Error>;
type Error = String;
type Value = usize;
type Image<T> = Vec<Array2D<T>>;

#[derive(Copy, Clone, Eq, PartialEq)]
struct Dimensions {
    width: usize,
    height: usize,
}

pub fn main() {
    let answer1 = solve1(INPUT, default_dimensions());
    let answer2 = solve2(INPUT, default_dimensions());
    println!("{:?}", answer1);
    println!("{:?}", answer2);
}

fn solve1(input: &str, dimensions: Dimensions) -> Result<Value> {
    let image = parse_input(input, dimensions)?;
    let chosen_layer = image
        .into_iter()
        .min_by_key(|layer| count_in_layer(layer, 0))
        .ok_or_else(|| "No layers".to_string())?;
    let num_ones = count_in_layer(&chosen_layer, 1);
    let num_twos = count_in_layer(&chosen_layer, 2);
    Ok(num_ones * num_twos)
}

fn solve2(_input: &str, _dimensions: Dimensions) -> Result<Value> {
    // let _ = parse_input(input, dimensions);
    Err("Nope".to_string())
}

fn count_in_layer(layer: &Array2D<Value>, goal: Value) -> usize {
    layer
        .elements_row_major_iter()
        .filter(|&&v| v == goal)
        .count()
}

fn parse_input(input: &str, dimensions: Dimensions) -> Result<Image<Value>> {
    let digits = input
        .trim()
        .chars()
        .map(|c| c.to_digit(BASE).map(|d| d as usize))
        .collect::<Option<Vec<_>>>()
        .ok_or_else(|| "Invalid digit".to_string())?;
    if digits.len() % dimensions.total_count() != 0 {
        return Err("Invalid dimensions".to_string());
    }
    let num_layers = digits.len() / dimensions.total_count();
    let layers = (0..num_layers)
        .map(|i| {
            let start = i * dimensions.total_count();
            let end = (i + 1) * dimensions.total_count();
            Array2D::from_row_major(&digits[start..end], dimensions.height, dimensions.width)
        })
        .collect();
    Ok(layers)
}

impl Dimensions {
    pub fn new(width: usize, height: usize) -> Self {
        Self { width, height }
    }

    pub fn total_count(&self) -> usize {
        self.width * self.height
    }
}

fn default_dimensions() -> Dimensions {
    Dimensions::new(DEFAULT_WIDTH, DEFAULT_HEIGHT)
}
