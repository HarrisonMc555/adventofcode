const INPUT: &str = include_str!("../../static/day06.txt");

const ROOT_LABEL: &str = "COM";

type Error = String;

#[derive(Debug, Eq, PartialEq)]
struct Node<T> {
    data: T,
    children: Vec<Node<T>>,
}

pub fn main() {
    let answer1 = solve1(INPUT);
    println!("{:?}", answer1);
}

fn solve1(input: &str) -> Result<usize, Error> {
    let pairs = parse_input(input)?;
    println!("{:?}", &pairs[..5]);
    Ok(pairs.len())
}

fn parse_input(input: &str) -> Result<Vec<(&str, &str)>, Error> {
    input.trim().lines().map(parse_line).collect()
}

fn parse_line(line: &str) -> Result<(&str, &str), Error> {
    let words = line.split(')').collect::<Vec<_>>();
    if let &[w1, w2] = &words[..] {
        Ok((w1, w2))
    } else {
        Err(format!("Invalid line {}", line))
    }
}

impl<T> Node<T> {
    fn new(data: T) -> Self {
        Node {
            data,
            children: Vec::new(),
        }
    }

    fn with_child(mut self, node: Node<T>) -> Self {
        self.add_child(node);
        self
    }

    fn add_child(&mut self, node: Node<T>) {
        self.children.push(node);
    }
}
