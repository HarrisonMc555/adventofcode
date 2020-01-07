use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::hash::Hash;

const INPUT: &str = include_str!("../../static/day14.txt");
const TARGET_NAME: &str = "FUEL";
const SOURCE_NAME: &str = "ORE";

type Count = u32;
type Result<T> = std::result::Result<T, String>;

#[derive(Debug, Eq, PartialEq)]
struct Reaction {
    reactants: Vec<Chemical>,
    product: Chemical,
}

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
struct Chemical {
    count: Count,
    name: String,
}

pub fn main() {
    let answer1 = solve1(INPUT);
    println!("{:?}", answer1);
    // let answer2 = solve2(INPUT);
    // println!("{:?}", answer2);
}

fn solve1(input: &str) -> Result<Count> {
    let reactions = parse_input(input);
    dbg!(reactions);
    Err("unimplemented".to_string())
}

fn num_required(target: &str, source: &str) -> Count {
    0
}

// fn linearization<T>(dependencies: HashMap<T, Vec<T>>) -> Vec<T>
// where
//     T: Hash + Eq,
// {
//     let mut visited = HashSet::new();
//     let mut post_nums = HashMap::new();
//     let mut post_num = 0;
//     let pre_visit = |_| {};
//     let post_visit = |item| {
//         post_nums.insert(item, post_num);
//         post_num += 1;
//     };

//     Vec::new()
// }

// fn depth_first_search<T, F>(graph: HashMap<T, Vec<T>>, explore: F)
// where
//     T: Hash + Eq,
//     F: FnMut(&T),
// {
//     let mut visited = HashSet::new();
//     let mut post_nums = HashMap::new();
//     let mut post_num = 0;
//     let pre_visit = |_| {};
//     let post_visit = |item| {
//         post_nums.insert(item, post_num);
//         post_num += 1;
//     };

//     Vec::new()
// }

// fn explore<'a, T, PreF, PostF>(
//     item: &'a T,
//     graph: HashMap<T, Vec<T>>,
//     visited: &mut HashSet<&'a T>,
//     pre_visit: PreF,
//     post_visit: PostF,
// ) where
//     T: Hash + Eq,
//     PreF: FnMut(&T),
//     PostF: FnMut(&T),
// {
//     visited.insert(item);
//     pre_visit(item);
//     if let Some(connections) = graph.get(item) {
//         for next in connections {
//             if !visited.contains(item) {
//                 explore(next, graph, visited, pre_visit, post_visit);
//             }
//         }
//     }
//     post_visit(item);
// }

fn parse_input(input: &str) -> Result<Vec<Reaction>> {
    input
        .lines()
        .map(parse_line)
        .collect::<Option<Vec<_>>>()
        .ok_or_else(|| "Invalid input".to_string())
}

fn parse_line(line: &str) -> Option<Reaction> {
    lazy_static! {
        static ref LINE_RE: Regex = Regex::new(r"(.+)=>(.*)").unwrap();
        static ref REACTANTS_RE: Regex = Regex::new(r"(.+)(,(.*))*").unwrap();
    }
    let line_cap = LINE_RE.captures(line)?;
    let reactants_cap = &line_cap[1];
    let product_cap = &line_cap[2];
    let reactants = reactants_cap
        .split(", ")
        .map(parse_chemical)
        .collect::<Option<Vec<_>>>()?;
    let product = parse_chemical(product_cap)?;
    Some(Reaction::new(reactants, product))
}

fn parse_chemical(chemical: &str) -> Option<Chemical> {
    lazy_static! {
        static ref CHEMICAL_RE: Regex = Regex::new(r" *(\d+) +(\w+)").unwrap();
    }
    let cap = CHEMICAL_RE.captures(chemical)?;
    let count = cap[1].parse::<Count>().ok()?;
    let name = &cap[2];
    Some(Chemical::new(count, name))
}

impl Chemical {
    pub fn new(count: Count, name: &str) -> Self {
        Chemical {
            count,
            name: name.to_string(),
        }
    }
}

impl Reaction {
    pub fn new(reactants: Vec<Chemical>, product: Chemical) -> Self {
        Reaction { reactants, product }
    }
}
