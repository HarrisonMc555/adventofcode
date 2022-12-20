use std::fmt::{Display, Formatter};
use std::str::FromStr;

use lazy_static::lazy_static;
use regex::{Captures, Regex};

use crate::days::{Day, Debug, Example, Part};
use crate::debug_println;

const DEBUG: bool = true;

pub struct Day19;

impl Day for Day19 {
    fn number(&self) -> u32 {
        19
    }

    fn run(&self, part: Part, example: Example, debug: Debug) {
        let answer = match part {
            Part::Part1 => self.part1(example, debug),
            Part::Part2 => self.part2(example, debug),
        };
        println!("{}", answer);
    }
}

impl Day19 {
    fn part1(&self, _example: Example, _debug: Debug) -> usize {
        todo!()
    }

    fn part2(&self, _example: Example, _debug: Debug) -> usize {
        todo!()
    }
}

#[derive(Debug, Eq, PartialEq)]
struct Blueprint {
    id: usize,
    robot_recipes: RobotRecipes,
}

#[derive(Debug, Eq, PartialEq)]
struct RobotRecipes([Costs; Resource::variant_count()]);

#[derive(Debug, Eq, PartialEq)]
struct RobotRecipe {
    robot_resource: Resource,
    costs: Costs,
}

#[derive(Debug, Eq, PartialEq)]
struct Costs([usize; Resource::variant_count()]);

#[derive(Debug, Clone, Eq, PartialEq)]
struct Cost {
    amount: usize,
    resource: Resource,
}

#[derive(Ordinalize, Debug, Hash, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
enum Resource {
    Ore,
    Clay,
    Obsidian,
    Geode,
}

impl FromStr for Blueprint {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        debug_println!("Parsing \"{}\" as {}", s, std::any::type_name::<Self>());
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^Blueprint (\d+):(.*)$").unwrap();
        }
        let caps = RE
            .captures(s.trim())
            .ok_or(format!("Does not match blueprint regex: \"{}\"", s))?;
        let id: usize = parse_group::<usize>(&caps, 1).map_err(|e| e.to_string())?;
        let robot_recipes = parse_group(&caps, 2)?;
        Ok(Blueprint { id, robot_recipes })
    }
}

impl FromStr for RobotRecipes {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        debug_println!("Parsing \"{}\" as {}", s, std::any::type_name::<Self>());
        let robot_recipes = s
            .trim()
            .split(".")
            .filter(|s| !s.is_empty())
            .map(|c| c.parse::<RobotRecipe>())
            .collect::<Result<Vec<_>, _>>()?;
        let expected_resources = Resource::variants().into_iter();
        let actual_resources = robot_recipes
            .iter()
            .map(|robot_recipe| robot_recipe.robot_resource);
        for (index, (expected_resource, actual_resource)) in
            expected_resources.zip(actual_resources).enumerate()
        {
            if expected_resource != actual_resource {
                return Err(format!(
                    "Expected resource {:?} at index {} but found {:?}",
                    expected_resource, index, actual_resource
                ));
            }
        }
        let costs = robot_recipes
            .into_iter()
            .map(|robot_recipe| robot_recipe.costs)
            .collect::<Vec<_>>();
        let array = costs.try_into().map_err(|costs: Vec<_>| {
            format!(
                "There should have been {} elements but there were {}",
                Resource::variant_count(), costs.len()
            )
        })?;
        Ok(RobotRecipes(array))
    }
}

impl FromStr for RobotRecipe {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        debug_println!("Parsing \"{}\" as {}", s, std::any::type_name::<Self>());
        lazy_static! {
            static ref RE: Regex = Regex::new(r"Each (\w+) robot costs ([^.]+)\.?").unwrap();
        }
        let caps = RE
            .captures(s.trim())
            .ok_or_else(|| does_not_match_regex(&*RE, s))?;
        let robot_resource = parse_group(&caps, 1)?;
        let costs = parse_group(&caps, 2)?;
        Ok(RobotRecipe {
            robot_resource,
            costs,
        })
    }
}

impl FromStr for Costs {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        debug_println!("Parsing \"{}\" as {}", s, std::any::type_name::<Self>());
        let mut costs = [0, 0, 0, 0];
        for cost in s.trim().split(" and ").map(|c| c.parse()) {
            let cost: Cost = cost?;
            let index = cost.resource.ordinal() as usize;
            let amount = &mut costs[index];
            if *amount > 0 {
                return Err(format!(
                    "Multiple amounts specified for {:?}",
                    cost.resource
                ));
            }
            *amount = cost.amount;
        }
        if costs.iter().all(|amount| *amount == 0) {
            return Err("Empty costs string".to_string());
        }
        Ok(Costs(costs))
    }
}

impl FromStr for Cost {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        debug_println!("Parsing \"{}\" as {}", s, std::any::type_name::<Self>());
        lazy_static! {
            static ref RE: Regex = Regex::new(r"(\d+) (\w+)").unwrap();
        }
        let caps = RE
            .captures(s.trim())
            .ok_or_else(|| does_not_match_regex(&*RE, s))?;
        let amount = parse_group::<usize>(&caps, 1).map_err(|e| e.to_string())?;
        let resource = parse_group(&caps, 2)?;
        Ok(Cost { amount, resource })
    }
}

impl FromStr for Resource {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        debug_println!("Parsing \"{}\" as {}", s, std::any::type_name::<Self>());
        Ok(match s {
            "ore" => Resource::Ore,
            "clay" => Resource::Clay,
            "obsidian" => Resource::Obsidian,
            "geode" => Resource::Geode,
            _ => return Err(format!("Invalid resource string \"{}\"", s)),
        })
    }
}

fn parse_group<T: FromStr>(captures: &Captures, group: usize) -> Result<T, T::Err> {
    captures.get(group).unwrap().as_str().parse()
}

enum IterToArrayError<const N: usize> {
    TooFewElements(usize),
    TooManyElements,
}

fn iter_to_array<T, const N: usize>(mut iter: T) -> Result<[T::Item; N], IterToArrayError<N>>
where
    T: Iterator,
{
    let mut vec = Vec::with_capacity(N);
    for index in 0..N {
        vec[index] = iter
            .next()
            .ok_or_else(|| IterToArrayError::TooFewElements(index))?;
    }
    vec.try_into()
        .map_err(|_| IterToArrayError::TooManyElements)
}

impl<const N: usize> Display for IterToArrayError<N> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            IterToArrayError::TooFewElements(num_elements) => write!(
                f,
                "There were only {} elements but should have been {}",
                num_elements, N
            ),
            IterToArrayError::TooManyElements => write!(f, "There were more than {} elements", N),
        }
    }
}

fn does_not_match_regex(regex: &Regex, string: &str) -> String {
    format!(
        "Input string \"{}\" does not match cost regex: \"{}\"",
        string,
        regex.as_str()
    )
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_examples_part1() {
        let actual1 = "Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.".parse().unwrap();
        let expected1 = Blueprint {
            id: 1,
            robot_recipes: RobotRecipes([
                Costs([4, 0, 0, 0]),
                Costs([2, 0, 0, 0]),
                Costs([3, 14, 0, 0]),
                Costs([2, 0, 7, 0]),
            ]),
        };
        assert_eq!(expected1, actual1);

        let actual2 = "Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian.".parse().unwrap();
        let expected2 = Blueprint {
            id: 2,
            robot_recipes: RobotRecipes([
                Costs([2, 0, 0, 0]),
                Costs([3, 0, 0, 0]),
                Costs([3, 8, 0, 0]),
                Costs([3, 0, 12, 0]),
            ]),
        };
        assert_eq!(expected2, actual2);
    }

    #[test]
    fn test_real_part1() {
        // assert_eq!(0, Day19.part1(Example::Real, Debug::NotDebug));
    }

    #[test]
    fn test_examples_part2() {
        // assert_eq!(0, Day19.part2(Example::Example, Debug::NotDebug));
    }

    #[test]
    fn test_real_part2() {
        // assert_eq!(0, Day19.part2(Example::Real, Debug::NotDebug));
    }
}
