use crate::util::graph::Graph;
use regex::Regex;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::hash::Hash;

const INPUT: &str = include_str!("../../static/day14.txt");
const TARGET_NAME: &str = "FUEL";
const TARGET_COUNT: Count = 1;
const SOURCE_NAME: &str = "ORE";
const SOURCE_COUNT: Count = 1_000_000_000_000;

type Count = u64;
type Result<T> = std::result::Result<T, String>;
type ChemicalName = String;

#[derive(Debug, Eq, PartialEq, Clone)]
struct Reaction {
    reactants: Vec<ChemicalCount>,
    product: ChemicalCount,
}

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
struct ChemicalCount {
    count: Count,
    name: ChemicalName,
}

pub fn main() {
    let answer1 = solve1(INPUT);
    println!("{:?}", answer1);
    let answer2 = solve2(INPUT);
    println!("{:?}", answer2);
}

fn solve1(input: &str) -> Result<Count> {
    let reactions = parse_input(input)?;
    let product_to_reaction = get_product_to_reaction(&reactions);
    let graph = reactions_to_graph(&reactions);
    let ordered_chemicals = graph
        .linearization()
        .ok_or_else(|| "Acyclic graph found".to_string())?;
    let target = (TARGET_NAME.to_string(), TARGET_COUNT);
    let chemical_to_count = get_chemical_to_count(&product_to_reaction, &ordered_chemicals, target);
    let source_count = chemical_to_count
        .get(SOURCE_NAME)
        .ok_or_else(|| format!("No {} found", SOURCE_NAME))?;
    Ok(*source_count)
}

fn solve2(input: &str) -> Result<Count> {
    let reactions = parse_input(input)?;
    let product_to_reaction = get_product_to_reaction(&reactions);
    let graph = reactions_to_graph(&reactions);
    let ordered_chemicals = graph
        .linearization()
        .ok_or_else(|| "Acyclic graph found".to_string())?;
    binary_search(
        |target_count| {
            *get_chemical_to_count(
                &product_to_reaction,
                &ordered_chemicals,
                (TARGET_NAME.to_string(), target_count),
            )
            .get(SOURCE_NAME)
            .unwrap()
        },
        |source_count| source_count < SOURCE_COUNT,
        1,
        SOURCE_COUNT,
    )
}

fn get_chemical_to_count(
    product_to_reaction: &HashMap<ChemicalName, Reaction>,
    ordered_chemicals: &[ChemicalName],
    (target_name, target_count): (ChemicalName, Count),
) -> HashMap<ChemicalName, Count> {
    let mut chemicals_needed = HashMap::new();
    chemicals_needed.insert(target_name, target_count);
    for chemical in ordered_chemicals {
        let reaction = match product_to_reaction.get(chemical) {
            Some(reaction) => reaction,
            // If there's no way to produce this chemical, we assume we have
            // unlimited quantities of it (and assume that we already know how
            // much we need).
            None => continue,
        };
        let needed = chemicals_needed.get(chemical).cloned().unwrap_or(0);
        let num_reactions = divide_round_up(needed, reaction.product.count);
        for reactant in reaction.reactants.iter() {
            let count = chemicals_needed.entry(reactant.name.clone()).or_insert(0);
            *count += num_reactions * reactant.count;
        }
    }
    chemicals_needed
}

fn get_product_to_reaction(reactions: &[Reaction]) -> HashMap<ChemicalName, Reaction> {
    reactions
        .iter()
        .map(|reaction: &Reaction| (reaction.product.name.to_string(), reaction.clone()))
        .collect()
}

fn reactions_to_graph(reactions: &[Reaction]) -> Graph<ChemicalName> {
    let edges = reactions
        .iter()
        .map(|reaction| {
            (
                reaction.product.name.clone(),
                reaction.reactants.iter().map(|r| r.name.clone()).collect(),
            )
        })
        .collect::<HashMap<_, _>>();
    Graph::from_edge_map(edges)
}

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

fn parse_chemical(chemical: &str) -> Option<ChemicalCount> {
    lazy_static! {
        static ref CHEMICAL_RE: Regex = Regex::new(r" *(\d+) +(\w+)").unwrap();
    }
    let cap = CHEMICAL_RE.captures(chemical)?;
    let count = cap[1].parse::<Count>().ok()?;
    let name = &cap[2];
    Some(ChemicalCount::new(count, name))
}

impl ChemicalCount {
    pub fn new(count: Count, name: &str) -> Self {
        ChemicalCount {
            count,
            name: name.to_string(),
        }
    }
}

impl Reaction {
    pub fn new(reactants: Vec<ChemicalCount>, product: ChemicalCount) -> Self {
        Reaction { reactants, product }
    }
}

fn binary_search<Map, Check>(
    map: Map,
    check: Check,
    mut low: Count,
    mut high: Count,
) -> Result<Count>
where
    Map: Fn(Count) -> Count,
    Check: Fn(Count) -> bool,
{
    loop {
        match low.cmp(&high) {
            Ordering::Less => (),
            Ordering::Equal => return Ok(low),
            Ordering::Greater => return Err("Bounds crossed, nothing found".to_string()),
        }
        let choice = divide_round_up(low + high, 2);
        let value = map(choice);
        if check(value) {
            low = choice;
        } else {
            high = choice - 1;
        }
    }
}

fn divide_round_up(x: Count, y: Count) -> Count {
    (x + y - 1) / y
}
