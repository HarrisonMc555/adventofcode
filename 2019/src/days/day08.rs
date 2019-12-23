use array2d::Array2D;

const INPUT: &str = include_str!("../../static/day08.txt");

const BASE: u32 = 10;
const DEFAULT_WIDTH: usize = 25;
const DEFAULT_HEIGHT: usize = 6;

const BLACK: Value = 0;
const WHITE: Value = 1;
const TRANSPARENT: Value = 2;

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
    let answer1 = solve1(INPUT, default_dimensions()).unwrap();
    println!("{}", answer1);
    let answer2 = solve2(INPUT, default_dimensions()).unwrap();
    print_message(&answer2);
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

fn solve2(input: &str, dimensions: Dimensions) -> Result<Array2D<Value>> {
    let image = parse_input(input, dimensions)?;
    let merged_layer = merge_layers(&image);
    Ok(merged_layer)
}

fn count_in_layer(layer: &Array2D<Value>, goal: Value) -> usize {
    layer
        .elements_row_major_iter()
        .filter(|&&v| v == goal)
        .count()
}

fn print_message(layer: &Array2D<Value>) {
    for row_iter in layer.rows_iter() {
        for element in row_iter {
            let c = match *element {
                TRANSPARENT => ' ',
                BLACK => '#',
                WHITE => ' ',
                _ => '?',
            };
            print!("{}", c);
        }
        println!();
    }
}

fn merge_layers(image: &[Array2D<Value>]) -> Array2D<Value> {
    let first_layer = match image.get(0) {
        Some(layer) => layer,
        None => return Array2D::filled_with(TRANSPARENT, 0, 0),
    };
    let num_rows = first_layer.num_rows();
    let num_columns = first_layer.num_columns();
    let num_elements = first_layer.num_elements();
    let pixels = (0..num_elements).map(|index| merge_pixel(image, index));
    Array2D::from_iter_row_major(pixels, num_rows, num_columns).unwrap()
}

fn merge_pixel(image: &[Array2D<Value>], row_major_index: usize) -> Value {
    image
        .iter()
        .flat_map(|layer| layer.get_row_major(row_major_index))
        .filter(|&&pixel| pixel != TRANSPARENT)
        .copied()
        .next()
        .unwrap_or(TRANSPARENT)
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
                .unwrap()
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn answer1() {
        assert_eq!(solve1(INPUT, default_dimensions()), Ok(1072));
    }

    #[test]
    fn answer2() {
        // Need to look at the message, so no testing here
    }
}
