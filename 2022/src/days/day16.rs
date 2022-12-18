use std::collections::HashMap;
use std::fmt::{Display, Formatter};

use lazy_static::lazy_static;
use regex::Regex;

use crate::days::{Day, Debug, Example, Part};

pub struct Day16;

impl Day for Day16 {
    fn number(&self) -> u32 {
        16
    }

    fn run(&self, part: Part, example: Example, debug: Debug) {
        let answer = match part {
            Part::Part1 => self.part1(example, debug),
            Part::Part2 => self.part2(example, debug),
        };
        println!("{}", answer);
    }
}

impl Day16 {
    fn part1(&self, _example: Example, _debug: Debug) -> usize {
        todo!()
    }

    fn part2(&self, _example: Example, _debug: Debug) -> usize {
        todo!()
    }
}

type ValveMap = HashMap<ValveID, Valve>;

#[derive(Debug, Eq, PartialEq)]
struct Valve {
    id: ValveID,
    flow_rate: usize,
    connections: Vec<ValveID>,
}

const VALVE_ID_LEN: usize = 2;
#[derive(Debug, Hash, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
struct ValveID([char; VALVE_ID_LEN]);

#[derive(Debug, Eq, PartialEq)]
struct WorldState {
    location_valve_id: ValveID,
    valve_states: HashMap<ValveID, ValveState>,
    pressure_relieved: usize,
    flow_rate: usize,
}

#[derive(Debug, Hash, Default, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
enum ValveState {
    #[default]
    Closed,
    Open,
}

#[derive(Debug)]
enum ExploreResult {
    OutOfTime(WorldState),
    StillExploring(WorldState),
}

fn find_max_flow_rate(valve_map: ValveMap, start: ValveID, num_minutes: usize) -> usize {
    let mut state = WorldState::new(valve_map.values().map(|valve| valve.id), start);

    fn step(valve_map: &ValveMap, mut state: WorldState) -> ExploreResult {
        todo!()
    }

    todo!()
}

impl WorldState {
    fn new<T: Iterator<Item = ValveID>>(valve_ids: T, location_valve_id: ValveID) -> Self {
        let valve_states = valve_ids
            .map(|valve_id| (valve_id, ValveState::Closed))
            .collect();
        WorldState {
            location_valve_id,
            valve_states,
            pressure_relieved: 0,
            flow_rate: 0,
        }
    }
}

fn parse_valve_map(text: &str) -> Option<ValveMap> {
    text.trim()
        .split('\n')
        .map(|text| Valve::parse(text).map(|valve| (valve.id.clone(), valve)))
        .collect()
}

impl Valve {
    fn parse(text: &str) -> Option<Self> {
        lazy_static! {
            static ref RE: Regex =
                Regex::new(r"^Valve (\w+) has flow rate=(\d+); tunnels lead to valves (.*)$")
                    .unwrap();
        }

        let caps = RE.captures(text)?;
        let id = caps.get(1).unwrap().as_str().try_into().ok()?;
        let flow_rate = caps.get(2).unwrap().as_str().parse().ok()?;
        let connections = caps
            .get(3)
            .unwrap()
            .as_str()
            .split(", ")
            .map(|valve_id| valve_id.try_into().ok())
            .collect::<Option<_>>()?;

        Some(Valve {
            id,
            flow_rate,
            connections,
        })
    }
}

impl TryFrom<&str> for ValveID {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut chars = value.chars();
        let first = chars.next().ok_or(())?;
        let second = chars.next().ok_or(())?;
        if chars.next().is_some() {
            return Err(());
        }
        Ok(ValveID([first, second]))
    }
}

impl Display for ValveID {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let [a, b] = self.0;
        write!(f, "{}{}", a, b)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse() {
        let actual =
            Valve::parse("Valve AA has flow rate=0; tunnels lead to valves DD, II, BB").unwrap();
        let expected = Valve {
            id: v("AA"),
            flow_rate: 0,
            connections: vec![v("DD"), v("II"), v("BB")],
        };
        assert_eq!(expected, actual);

        let actual =
            Valve::parse("Valve BB has flow rate=13; tunnels lead to valves CC, AA").unwrap();
        let expected = Valve {
            id: v("BB"),
            flow_rate: 13,
            connections: vec![v("CC"), v("AA")],
        };
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_examples_part1() {
        // assert_eq!(0, Day16.part1(Example::Example, Debug::NotDebug));
    }

    #[test]
    fn test_real_part1() {
        // assert_eq!(0, Day16.part1(Example::Real, Debug::NotDebug));
    }

    #[test]
    fn test_examples_part2() {
        // assert_eq!(0, Day16.part2(Example::Example, Debug::NotDebug));
    }

    #[test]
    fn test_real_part2() {
        // assert_eq!(0, Day16.part2(Example::Real, Debug::NotDebug));
    }

    fn v(valve_id: &str) -> ValveID {
        valve_id.try_into().unwrap()
    }
}
