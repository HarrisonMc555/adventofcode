use std::collections::{HashMap, VecDeque};
use std::fmt::{Display, Formatter};
use std::str::FromStr;

use lazy_static::lazy_static;
use regex::Regex;

use crate::days::{Day, Debug, Example, Part};
use crate::{debug_print, debug_println};

const DEBUG: bool = false;

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
    fn part1(&self, example: Example, _debug: Debug) -> usize {
        let valve_map = parse_valve_map(&self.read_file(example)).unwrap();
        let info = Info::new(valve_map);
        State::find_max_flow_rate(&info, START_VALVE, NUM_MINUTES)
    }

    fn part2(&self, _example: Example, _debug: Debug) -> usize {
        todo!()
    }
}

const NUM_MINUTES: usize = 30;
const START_VALVE: ValveID = ValveID([b'A', b'A']);

type ValveMap = HashMap<ValveID, Valve>;

#[derive(Debug, Eq, PartialEq)]
struct Valve {
    id: ValveID,
    flow_rate: usize,
    connections: Vec<ValveID>,
}

const VALVE_ID_LEN: usize = 2;
#[derive(Hash, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
struct ValveID([u8; VALVE_ID_LEN]);

#[derive(Debug)]
struct Info {
    valve_map: ValveMap,
    distances: HashMap<ValveID, HashMap<ValveID, usize>>,
    highest_pressure_valves: Vec<ValveID>,
}

#[derive(Debug, Clone)]
struct State {
    remaining_valves: Vec<ValveID>,
    cur_total_pressure: usize,
    cur_pressure_per_minute: usize,
    remaining_minutes: usize,
    cur_valve: ValveID,
    #[cfg(test)]
    start_minutes: usize,
}

#[derive(Debug, Clone)]
struct State2 {
    remaining_valves: Vec<ValveID>,
    cur_total_pressure: usize,
    cur_pressure_per_minute: usize,
    remaining_minutes: usize,
    searcher_states: Vec<(ValveID, usize)>,
    #[cfg(test)]
    start_minutes: usize,
}

impl State {
    pub fn find_max_flow_rate(info: &Info, start_valve: ValveID, num_minutes: usize) -> usize {
        let state = State {
            remaining_valves: info.highest_pressure_valves.clone(),
            cur_total_pressure: 0,
            cur_pressure_per_minute: 0,
            remaining_minutes: num_minutes,
            cur_valve: start_valve,
            #[cfg(test)]
            start_minutes: num_minutes,
        };
        debug_println!("+--------------------------------------+");
        debug_println!("| Finding max flow rate for {: >2} minutes |", num_minutes);
        debug_println!("+--------------------------------------+");
        state.find_max_flow_rate_helper(info, 0)
    }

    fn find_max_flow_rate_helper(self, info: &Info, best: usize) -> usize {
        #[cfg(test)]
        let prefix = "  ".repeat(self.start_minutes - self.remaining_minutes);
        #[cfg(not(test))]
        let prefix = "".to_string();
        debug_println!(
            "{}At {} with {} minutes left.",
            prefix,
            self.cur_valve,
            self.remaining_minutes
        );
        debug_print!("{}Old best: {}, new best: ", prefix, best);
        let mut best = best
            .max(self.cur_total_pressure + self.cur_pressure_per_minute * self.remaining_minutes);
        debug_println!("{}", best);
        if self.remaining_minutes == 0 {
            debug_println!("{}Done, returning best: {}", prefix, best);
            return best;
        }
        if self.theoretical_max(info) <= best {
            debug_println!(
                "{}Theoretical max is {}, quitting early",
                prefix,
                self.theoretical_max(info)
            );
            return best;
        }

        for dest_valve_id in self.remaining_valves.iter().copied() {
            let distance = info.distances[&self.cur_valve][&dest_valve_id];
            let num_minutes_required = distance + 1;
            if self.remaining_minutes < num_minutes_required + 1 {
                debug_println!(
                    "{}Not enough time to visit {} and get something out of it (requires {} minutes, we only have {} \
                    left)",
                    prefix,
                    dest_valve_id,
                    num_minutes_required,
                    self.remaining_minutes
                );
                continue;
            }
            let dest_valve_pressure = info.valve_map[&dest_valve_id].flow_rate;
            let next_valves = self
                .remaining_valves
                .iter()
                .copied()
                .filter(|valve_id| *valve_id != dest_valve_id)
                .collect();

            let next_state = State {
                remaining_valves: next_valves,
                cur_total_pressure: self.cur_total_pressure
                    + self.cur_pressure_per_minute * num_minutes_required,
                cur_pressure_per_minute: self.cur_pressure_per_minute + dest_valve_pressure,
                remaining_minutes: self.remaining_minutes - num_minutes_required,
                cur_valve: dest_valve_id,
                #[cfg(test)]
                start_minutes: self.start_minutes,
            };
            debug_println!(
                "{}Visiting {} next, new state: {:?}",
                prefix,
                dest_valve_id,
                next_state
            );
            best = best.max(next_state.find_max_flow_rate_helper(info, best));
        }
        best
    }

    fn theoretical_max(&self, info: &Info) -> usize {
        let max_num_open = self.remaining_minutes / 2;
        let flow_rates = self
            .remaining_valves
            .iter()
            .map(|valve_id| info.valve_map[valve_id].flow_rate)
            .take(max_num_open);
        let mut total_pressure = self.cur_total_pressure;
        let mut pressure_per_minute = self.cur_pressure_per_minute;
        let mut remaining_minutes = self.remaining_minutes;
        for flow_rate in flow_rates {
            if remaining_minutes <= 1 {
                break;
            }
            total_pressure += pressure_per_minute * 2;
            pressure_per_minute += flow_rate;
            remaining_minutes -= 2;
        }
        total_pressure += pressure_per_minute * remaining_minutes;
        total_pressure
    }
}

impl State2 {
    pub fn find_max_flow_rate(
        info: &Info,
        start_valves: Vec<ValveID>,
        num_minutes: usize,
    ) -> usize {
        let searcher_states = start_valves
            .into_iter()
            .map(|valve_id| (valve_id, 0))
            .collect();
        let state = State2 {
            remaining_valves: info.highest_pressure_valves.clone(),
            cur_total_pressure: 0,
            cur_pressure_per_minute: 0,
            remaining_minutes: num_minutes,
            searcher_states,
            #[cfg(test)]
            start_minutes: num_minutes,
        };
        debug_println!("+--------------------------------------+");
        debug_println!("| Finding max flow rate for {: >2} minutes |", num_minutes);
        debug_println!("+--------------------------------------+");
        state.find_max_flow_rate_helper(info, 0)
    }

    fn find_max_flow_rate_helper(self, info: &Info, best: usize) -> usize {
        #[cfg(test)]
        let prefix = "  ".repeat(self.start_minutes - self.remaining_minutes);
        #[cfg(not(test))]
        let prefix = "".to_string();
        debug_println!(
            "{}{} minutes left, searchers: {:?}.",
            prefix,
            self.remaining_minutes,
            self.searcher_states,
        );
        debug_print!("{}Old best: {}, new best: ", prefix, best);
        let mut best = best
            .max(self.cur_total_pressure + self.cur_pressure_per_minute * self.remaining_minutes);
        debug_println!("{}", best);
        if self.remaining_minutes == 0 {
            debug_println!("{}Done, returning best: {}", prefix, best);
            return best;
        }
        if self.theoretical_max(info) <= best {
            debug_println!(
                "{}Theoretical max is {}, quitting early",
                prefix,
                self.theoretical_max(info)
            );
            return best;
        }

        let mut min_time = None;
        for (cur_valve, num_minutes) in self.searcher_states.iter().copied() {
            if num_minutes > 0 {
                min_time = Some(num_minutes.min(min_time.unwrap_or(num_minutes)));
                continue;
            }


        }

        for dest_valve_id in self.remaining_valves.iter().copied() {
            let distance = info.distances[&self.cur_valve1][&dest_valve_id];
            let num_minutes_required = distance + 1;
            if self.remaining_minutes < num_minutes_required + 1 {
                debug_println!(
                    "{}Not enough time to visit {} and get something out of it (requires {} minutes, we only have {} \
                    left)",
                    prefix,
                    dest_valve_id,
                    num_minutes_required,
                    self.remaining_minutes
                );
                continue;
            }
            let dest_valve_pressure = info.valve_map[&dest_valve_id].flow_rate;
            let next_valves = self
                .remaining_valves
                .iter()
                .copied()
                .filter(|valve_id| *valve_id != dest_valve_id)
                .collect();

            let next_state = State2 {
                remaining_valves: next_valves,
                cur_total_pressure: self.cur_total_pressure
                    + self.cur_pressure_per_minute * num_minutes_required,
                cur_pressure_per_minute: self.cur_pressure_per_minute + dest_valve_pressure,
                remaining_minutes: self.remaining_minutes - num_minutes_required,
                cur_valve1: dest_valve_id,
                cur_valve2: dest_valve_id,
                #[cfg(test)]
                start_minutes: self.start_minutes,
            };
            debug_println!(
                "{}Visiting {} next, new state: {:?}",
                prefix,
                dest_valve_id,
                next_state
            );
            best = best.max(next_state.find_max_flow_rate_helper(info, best));
        }
        best
    }

    fn inner(&self, info: &Info, best: usize, searcher_index: usize) -> usize {
        #[cfg(test)]
            let prefix = "  ".repeat(self.start_minutes - self.remaining_minutes);
        #[cfg(not(test))]
            let prefix = "".to_string();

        let (cur_valve, cur_minutes) = self.searcher_states[searcher_index];
        debug_assert_eq!(0, cur_minutes);

        for dest_valve_id in self.remaining_valves.iter().copied() {

            let distance = info.distances[&cur_valve][&dest_valve_id];
            let num_minutes_required = distance + 1;
            if self.remaining_minutes < num_minutes_required + 1 {
                debug_println!(
                    "{}Not enough time to visit {} and get something out of it (requires {} minutes, we only have {} \
                    left)",
                    prefix,
                    dest_valve_id,
                    num_minutes_required,
                    self.remaining_minutes
                );
                continue;
            }
            let dest_valve_pressure = info.valve_map[&dest_valve_id].flow_rate;
            let next_valves = self
                .remaining_valves
                .iter()
                .copied()
                .filter(|valve_id| *valve_id != dest_valve_id)
                .collect();
            let mut searcher_states = self.searcher_states.clone();
            searcher_states[searcher_index].1 = todo!();

            let next_state = State2 {
                remaining_valves: next_valves,
                cur_total_pressure: self.cur_total_pressure
                    + self.cur_pressure_per_minute * num_minutes_required,
                cur_pressure_per_minute: self.cur_pressure_per_minute + dest_valve_pressure,
                remaining_minutes: self.remaining_minutes - num_minutes_required,
                cur_valve: dest_valve_id,
                #[cfg(test)]
                start_minutes: self.start_minutes,
            };
            debug_println!(
                "{}Visiting {} next, new state: {:?}",
                prefix,
                dest_valve_id,
                next_state
            );
            best = best.max(next_state.find_max_flow_rate_helper(info, best));

        }

        best
    }

    fn theoretical_max(&self, info: &Info) -> usize {
        let max_num_open = self.remaining_minutes / 2;
        let flow_rates = self
            .remaining_valves
            .iter()
            .map(|valve_id| info.valve_map[valve_id].flow_rate)
            .take(max_num_open);
        let mut total_pressure = self.cur_total_pressure;
        let mut pressure_per_minute = self.cur_pressure_per_minute;
        let mut remaining_minutes = self.remaining_minutes;
        for flow_rate in flow_rates {
            if remaining_minutes <= 1 {
                break;
            }
            total_pressure += pressure_per_minute * 2;
            pressure_per_minute += flow_rate;
            remaining_minutes -= 2;
        }
        total_pressure += pressure_per_minute * remaining_minutes;
        total_pressure
    }
}

impl Info {
    fn new(valve_map: ValveMap) -> Self {
        let distances = Info::get_distances(&valve_map);
        let highest_pressure_valves = Info::get_highest_pressure_valves(&valve_map);
        Info {
            valve_map,
            distances,
            highest_pressure_valves,
        }
    }

    fn get_distances(valve_map: &ValveMap) -> HashMap<ValveID, HashMap<ValveID, usize>> {
        valve_map
            .keys()
            .map(|valve_id| (*valve_id, Info::get_distances_from(valve_map, *valve_id)))
            .collect()
    }

    fn get_distances_from(valve_map: &ValveMap, valve_id: ValveID) -> HashMap<ValveID, usize> {
        let mut distances = HashMap::<ValveID, usize>::new();
        let mut process_queue = VecDeque::new();
        process_queue.push_back((valve_id, 0));
        debug_println!("Getting distances from {}", valve_id);
        while let Some((dest_valve_id, new_distance)) = process_queue.pop_front() {
            debug_println!(
                "== Popped {} from queue (distance {}) ==",
                dest_valve_id,
                new_distance
            );
            match distances.get(&dest_valve_id) {
                Some(old_distance) if new_distance < *old_distance => {
                    debug_println!(
                        "\tNew distance {} is less than old distance {}",
                        new_distance,
                        old_distance
                    );
                    distances.insert(dest_valve_id, new_distance);
                }
                Some(old_distance) => {
                    debug_println!(
                        "\tNew distance {} is not less than old distance {}",
                        new_distance,
                        old_distance
                    );
                    continue;
                }
                None => {
                    debug_println!("\tAdding new distance {}", new_distance);
                    distances.insert(dest_valve_id, new_distance);
                }
            }
            for next_dest in valve_map[&dest_valve_id].connections.iter() {
                debug_println!(
                    "\t\tQueueing up connection to {} (with distance {})",
                    next_dest,
                    new_distance + 1
                );
                process_queue.push_back((*next_dest, new_distance + 1));
            }
            debug_println!();
        }
        distances
    }

    fn get_highest_pressure_valves(valve_map: &ValveMap) -> Vec<ValveID> {
        let mut valves = valve_map
            .iter()
            .filter(|(_, valve)| valve.flow_rate > 0)
            .collect::<Vec<_>>();
        valves.sort_unstable_by_key(|(_, valve)| std::cmp::Reverse(valve.flow_rate));
        valves.into_iter().map(|(valve_id, _)| *valve_id).collect()
    }
}

fn parse_valve_map(text: &str) -> Result<ValveMap, &str> {
    text.lines()
        .map(|line| {
            Valve::parse(line)
                .map(|valve| (valve.id, valve))
                .ok_or(line)
        })
        .collect()
}

impl Valve {
    fn parse(text: &str) -> Option<Self> {
        lazy_static! {
            static ref RE: Regex =
                Regex::new(r"^Valve (\w+) has flow rate=(\d+); tunnels? leads? to valves? (.*)$")
                    .unwrap();
        }

        let caps = RE.captures(text)?;
        let id = caps.get(1).unwrap().as_str().parse().ok()?;
        let flow_rate = caps.get(2).unwrap().as_str().parse().ok()?;
        let connections = caps
            .get(3)
            .unwrap()
            .as_str()
            .split(", ")
            .map(|valve_id| valve_id.parse().ok())
            .collect::<Option<_>>()?;

        Some(Valve {
            id,
            flow_rate,
            connections,
        })
    }
}

impl FromStr for ValveID {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = s.chars();
        let first = chars.next().ok_or(())?;
        let second = chars.next().ok_or(())?;
        if chars.next().is_some() {
            return Err(());
        }
        if first.is_ascii_alphabetic() && second.is_ascii_alphabetic() {
            Ok(ValveID([first as u8, second as u8]))
        } else {
            Err(())
        }
    }
}

impl Display for ValveID {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let [a, b] = self.0;
        write!(f, "{}{}", a as char, b as char)
    }
}

impl std::fmt::Debug for ValveID {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
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

        assert!(parse_valve_map(include_str!("../../static/example16.txt")).is_ok());
    }

    #[test]
    fn test_get_highest_pressure_valves() {
        let valve_map = parse_valve_map(include_str!("../../static/example16.txt")).unwrap();
        let actual = Info::get_highest_pressure_valves(&valve_map);
        let expected = vec![v("HH"), v("JJ"), v("DD"), v("BB"), v("EE"), v("CC")];
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_get_distances_from() {
        let valve_map = parse_valve_map(include_str!("../../static/example16.txt")).unwrap();
        let distances = Info::get_distances_from(&valve_map, v("AA"));
        for valve_id in valve_map.keys().filter(|valve_id| **valve_id != v("AA")) {
            assert!(distances.contains_key(valve_id), "Contains {}", valve_id);
        }
    }

    #[test]
    fn test_get_distances() {
        let valve_map = parse_valve_map(include_str!("../../static/example16.txt")).unwrap();
        let actual = Info::get_distances(&valve_map);
        for (source, distances) in actual.iter() {
            debug_println!("From {}", source);
            for (dest, distance) in distances.iter() {
                debug_println!("\tTo {}: {}", dest, distance);
            }
            debug_println!();
        }
        for source in valve_map.keys() {
            for dest in valve_map.keys().filter(|valve_id| *valve_id != source) {
                assert!(actual[source].contains_key(dest));
            }
        }
        let a_map = &actual[&v("AA")];
        assert_eq!(1, a_map[&v("DD")]);
        assert_eq!(1, a_map[&v("II")]);
        assert_eq!(1, a_map[&v("BB")]);
        assert_eq!(2, a_map[&v("CC")]);
        assert_eq!(2, a_map[&v("EE")]);
        assert_eq!(2, a_map[&v("JJ")]);
        assert_eq!(3, a_map[&v("FF")]);
        assert_eq!(4, a_map[&v("GG")]);
        assert_eq!(5, a_map[&v("HH")]);
    }

    #[test]
    fn test_small() {
        let valve_map = parse_valve_map(include_str!("../../static/example16.txt")).unwrap();
        let info = Info::new(valve_map);
        let actual = |num_minutes| State::find_max_flow_rate(&info, START_VALVE, num_minutes);
        assert_eq!(0, actual(0)); // Nothing
        assert_eq!(0, actual(1)); // -> DD, open DD
        assert_eq!(0, actual(2)); // -> DD, open DD
        assert_eq!(20, actual(3)); // -> DD, open DD, +20
        assert_eq!(40, actual(4)); // -> DD, open DD, +20, +20
        assert_eq!(63, actual(5)); // -> DD, open DD, +20 -> EE, +20 open EE, +23
        assert_eq!(93, actual(6)); // -> DD, open DD, +20 -> AA, +20 -> BB, +20 -> open BB, +33
    }

    #[test]
    fn test_examples_part1() {
        assert_eq!(1651, Day16.part1(Example::Example, Debug::NotDebug));
    }

    #[test]
    fn test_real_part1() {
        assert_eq!(1767, Day16.part1(Example::Real, Debug::NotDebug));
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
        valve_id.parse().unwrap()
    }
}
