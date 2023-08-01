use std::{str::FromStr, collections::{HashSet, VecDeque}};

use crate::days::{Day, Debug, Example, Part};
use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;
pub struct Day11;

const DEBUG: bool = false;

const NOTHING_RELEVANT: &str = "nothing relevant";
const ADDITIONAL_ELEMENTS: [&str; 2] = ["elerium", "dilithium"];

impl Day for Day11 {
    fn number(&self) -> u32 {
        11
    }

    fn run(&self, part: Part, example: Example, debug: Debug) {
        let answer = match part {
            Part::Part1 => self.part1(example, debug),
            Part::Part2 => self.part2(example, debug),
        };
        println!("{}", answer);
    }
}

impl Day11 {
    fn part1(&self, example: Example, _debug: Debug) -> usize {
        let floors = parse_floors(&Day11.read_file(example)).unwrap();
        let state = State::new(floors);
        find_minimum_steps(state).unwrap()
    }

    fn part2(&self, example: Example, _debug: Debug) -> usize {
        let mut floors = parse_floors(&Day11.read_file(example)).unwrap();
        let first_floor = &mut floors[0];
        for element in ADDITIONAL_ELEMENTS {
            first_floor.push(Item::Generator(Element(element.to_owned())));
            first_floor.push(Item::Microchip(Element(element.to_owned())));
        }
        let state = State::new(floors);
        find_minimum_steps(state).unwrap()
    }
}

#[derive(Debug, Hash, Eq, PartialEq, Clone, Ord, PartialOrd)]
enum Item {
    Generator(Element),
    Microchip(Element),
}

#[derive(Debug, Hash, Eq, PartialEq, Clone, Ord, PartialOrd)]
struct Element(String);

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
struct State {
    elevator_index: usize,
    floors: Vec<Vec<Item>>,
}

fn find_minimum_steps(initial_state: State) -> Option<usize> {
    if initial_state.is_solution() {
        return Some(0);
    }
    let mut step = 0;
    let mut seen = HashSet::new();
    seen.insert(initial_state.clone());
    let mut queue = VecDeque::new();
    queue.push_back((initial_state, 0));
    while let Some((state, num_steps)) = queue.pop_front() {
        debug_println!("It takes {num_steps} steps to get to this state with hash {:08X}", hash(&state));
        debug_println!("{:?}", state);
        state.debug_print();
        debug_println!();
        debug_println!("Before adding next states, we have seen {} and queue contains {}", seen.len(), queue.len());
        // if step > 20 {
        //     break;
        // }
        step += 1;
        if step % 100 == 0 {
            println!("Step {step}, seen {}, queue {}", seen.len(), queue.len());
        }
        let next_num_steps = num_steps + 1;
        for next_state in state.possible_next_states() {
            if seen.contains(&next_state) {
                continue;
            }
            if next_state.is_solution() {
                return Some(next_num_steps);
            }
            seen.insert(next_state.clone());
            queue.push_back((next_state, next_num_steps));
        }
        debug_println!("After adding next states, we have seen {} and queue contains {}", seen.len(), queue.len());
        debug_println!();
    }
    None
}

#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone, Ord, PartialOrd)]
enum ElevatorDirection {
    Up,
    Down,
}

impl State {
    pub fn new(mut floors: Vec<Vec<Item>>) -> Self {
        for floor in floors.iter_mut() {
            floor.sort();
        }
        Self {
            elevator_index: 0,
            floors,
        }
    }

    pub fn possible_next_states(&self) -> Vec<State> {
        let mut next_states = Vec::new();
        if let Some(up_index) = self.next_elevator_index(ElevatorDirection::Up) {
            next_states.extend(self.possible_next_states_to_floor(up_index));
        };
        if let Some(down_index) = self.next_elevator_index(ElevatorDirection::Down) {
            next_states.extend(self.possible_next_states_to_floor(down_index));
        };
        next_states
    }

    pub fn is_solution(&self) -> bool {
        self.floors.iter().rev().skip(1).all(|floor| floor.is_empty())
    }

    fn next_elevator_index(&self, elevator_direction: ElevatorDirection) -> Option<usize> {
        match elevator_direction {
            ElevatorDirection::Up => Some(self.elevator_index + 1).filter(|index| *index < self.floors.len()),
            ElevatorDirection::Down => self.elevator_index.checked_sub(1),
        }
    }

    fn possible_next_states_to_floor(&self, next_elevator_index: usize) -> Vec<Self> {
        let num_items = self.floors[self.elevator_index].len();
        let mut next_states = Vec::new();
        for index in 0..num_items {
            let next_floors = Self::move_item_to_floor(&self.floors, index, self.elevator_index, next_elevator_index);
            let num_remaining_items = next_floors[self.elevator_index].len();
            for second_index in 0..num_remaining_items {
                let next_floors = Self::move_item_to_floor(&next_floors, second_index, self.elevator_index, next_elevator_index);
                let next_state = Self {
                    elevator_index: next_elevator_index,
                    floors: next_floors,
                };
                if next_state.is_valid() {
                    next_states.push(next_state);
                }
            }

            // Do this after the inner loop to avoid extra clone
            let next_state = Self {
                elevator_index: next_elevator_index,
                floors: next_floors,
            };
            if next_state.is_valid() {
                next_states.push(next_state);
            }

        }

        next_states
    }

    fn is_valid(&self) -> bool {
        self.floors.iter().all(|floor| Self::is_valid_floor(floor))
    }

    fn is_valid_floor(floor: &[Item]) -> bool {
        let num_generators = floor.iter().filter(|item| matches!(item, Item::Generator(_))).count();
        if num_generators == 0 {
            return true;
        }
        for item in floor {
            let Item::Microchip(element) = item else {
                continue;
            };
            let has_matching_generator = floor.iter().find(|item| matches!(item, Item::Generator(generator_element) if generator_element == element)).is_some();
            if !has_matching_generator {
                return false;
            }
        }
        true
    }
    
    fn move_item_to_floor(floors: &[Vec<Item>], item_index: usize, from_floor_index: usize, to_floor_index: usize) -> Vec<Vec<Item>> {
        let mut cur_floor_taken = floors[from_floor_index].clone();
        let item = cur_floor_taken.remove(item_index);
        let mut next_floor_added = floors[to_floor_index].clone();
        let pos = match next_floor_added.binary_search(&item) {
            Ok(pos) => pos,
            Err(pos) => pos,
        };
        next_floor_added.insert(pos, item);
        let mut next_floors = floors.to_owned();
        next_floors[from_floor_index] = cur_floor_taken.clone();
        next_floors[to_floor_index] = next_floor_added.clone();
        next_floors
    }

    fn debug_print(&self) {
        if !DEBUG {
            return;
        }
        let elements = self.floors.iter().flat_map(|floor| floor.iter()).map(|item| match item {
            Item::Generator(ref element) => element,
            Item::Microchip(ref element) => element,
        }).unique().sorted().collect::<Vec<_>>();

        for (index, floor) in self.floors.iter().enumerate().rev() {
            debug_print!("F{} ", index + 1);
            if index == self.elevator_index {
                debug_print!("E  ");
            } else {
                debug_print!(".  ");
            }
            for element in elements.iter().copied() {
                if floor.iter().find(|item| matches!(item, Item::Generator(generator_element) if generator_element == element)).is_some() {
                    debug_print!("{}G ", element.0.chars().next().unwrap().to_uppercase());
                } else {
                    debug_print!(".  ");
                }
                if floor.iter().find(|item| matches!(item, Item::Microchip(microchip_element) if microchip_element == element)).is_some() {
                    debug_print!("{}M ", element.0.chars().next().unwrap().to_uppercase());
                } else {
                    debug_print!(".  ");
                }
            }
            debug_println!();
        }
    }
}

fn parse_floors(text: &str) -> Result<Vec<Vec<Item>>, String> {
    text.lines().map(parse_floor).collect()
}

fn parse_floor(line: &str) -> Result<Vec<Item>, String> {
    lazy_static! {
        static ref LINE_RE: Regex = Regex::new(r"^The \w+ floor contains (?:a )?(.+)\.$").unwrap();
        static ref DIVIDER_RE: Regex = Regex::new(r"(, a |,? and a )").unwrap();
    }
    let contents = LINE_RE
        .captures(line)
        .ok_or_else(|| format!("Line \"{line}\" does not match line regular expression \"{}\"", LINE_RE.as_str()))?
        .get(1)
        .unwrap()
        .as_str();
    if contents == NOTHING_RELEVANT {
        return Ok(Vec::new());
    }
    DIVIDER_RE
        .split(contents)
        .map(|item| item.parse())
        .collect()
}

impl FromStr for Item {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref GENERATOR_RE: Regex = Regex::new(r"^(\w+) generator$").unwrap();
            static ref MICROCHIP_RE: Regex = Regex::new(r"(\w+)-compatible microchip").unwrap();
        }
        if let Some(caps) = GENERATOR_RE.captures(s) {
            let element = Element(caps.get(1).unwrap().as_str().to_owned());
            Ok(Item::Generator(element))
        } else if let Some(caps) = MICROCHIP_RE.captures(s) {
            let element = Element(caps.get(1).unwrap().as_str().to_owned());
            Ok(Item::Microchip(element))
        } else {
            Err(format!("Invalid item string \"{s}\""))
        }
    }
}

fn hash<T: std::hash::Hash>(value: T) -> u64 {
    use std::hash::Hasher;
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    value.hash(&mut hasher);
    hasher.finish()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse() {
        let floors = parse_floors(&Day11.read_file(Example::Example)).unwrap();
        let floor1 = vec![microchip("hydrogen"), microchip("lithium")];
        assert_eq!(floor1, floors[0]);
        let floor2 = vec![generator("hydrogen")];
        assert_eq!(floor2, floors[1]);
        let floor3 = vec![generator("lithium")];
        assert_eq!(floor3, floors[2]);
        let floor4: Vec<Item> = vec![];
        assert_eq!(floor4, floors[3]);
    }

    #[test]
    fn test_possible_next_states() {
        let floors = parse_floors(&Day11.read_file(Example::Example)).unwrap();
        let state = State::new(floors);
        let actual = state.possible_next_states();

        let mut next_state = state.clone();
        next_state.elevator_index = 1;
        let hydrogen_microchip = next_state.floors[0].remove(0);
        next_state.floors[1].push(hydrogen_microchip);
        let expected = vec![next_state];
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_examples_part1() {
        let floors = parse_floors(&Day11.read_file(Example::Example)).unwrap();
        let state = State::new(floors);
        let actual = find_minimum_steps(state).unwrap();
        assert_eq!(11, actual);
    }

    #[test]
    fn test_real_part1() {
        // assert_eq!(0, Day11.part1(Example::Real, Debug::NotDebug));
    }

    #[test]
    fn test_examples_part2() {}

    #[test]
    fn test_real_part2() {
        // assert_eq!(0, Day11.part2(Example::Real, Debug::NotDebug));
    }

    fn microchip(element: &str) -> Item {
        Item::Microchip(Element(element.to_owned()))
    }

    fn generator(element: &str) -> Item {
        Item::Generator(Element(element.to_owned()))
    }
}
