use std::cmp::Ordering;
use std::fmt::{Display, Formatter};
use std::iter::Peekable;

use itertools::Itertools;

use crate::days::{Day, Debug, Example, Part};
use crate::debug_println;

const DEBUG: bool = false;

pub struct Day13;

impl Day for Day13 {
    fn number(&self) -> u32 {
        13
    }

    fn run(&self, part: Part, example: Example, debug: Debug) {
        let answer = match part {
            Part::Part1 => self.part1(example, debug),
            Part::Part2 => self.part2(example, debug),
        };
        println!("{}", answer);
    }
}

impl Day13 {
    fn part1(&self, example: Example, _debug: Debug) -> usize {
        let packet_pairs = parse_packet_pairs(&self.read_file(example)).unwrap();
        packet_pairs
            .into_iter()
            .enumerate()
            .filter(|(_, (packet1, packet2))| List::is_ordered(packet1, packet2))
            .map(|(index, _)| index + 1)
            .sum()
    }

    fn part2(&self, example: Example, _debug: Debug) -> usize {
        use std::iter::once;
        let packet_pairs = parse_packet_pairs(&self.read_file(example)).unwrap();
        let mut packets = packet_pairs
            .iter()
            .flat_map(|(packet1, packet2)| once(packet1).chain(once(packet2)))
            .collect::<Vec<_>>();
        let divider_packet1 = create_divider_packet(DIVIDER_PACKET_NUM1);
        let divider_packet2 = create_divider_packet(DIVIDER_PACKET_NUM2);
        packets.push(&divider_packet1);
        packets.push(&divider_packet2);
        packets.sort();
        for packet in packets.iter() {
            debug_println!("{}", packet)
        }
        let index1 = index_of(&packets, &&divider_packet1).unwrap() + 1;
        let index2 = index_of(&packets, &&divider_packet2).unwrap() + 1;
        index1 * index2
    }
}

const DIVIDER_PACKET_NUM1: u32 = 2;
const DIVIDER_PACKET_NUM2: u32 = 6;

#[derive(Debug, Clone, Eq, PartialEq)]
enum Item {
    List(List),
    Integer(Integer),
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct List(Vec<Item>);
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
struct Integer(u32);

fn create_divider_packet(num: u32) -> List {
    List(vec![Item::List(List(vec![Item::Integer(Integer(num))]))])
}

impl List {
    fn is_ordered(list1: &List, list2: &List) -> bool {
        list1 <= list2
    }
}

impl PartialOrd for Item {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Item {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Item::Integer(integer1), Item::Integer(integer2)) => integer1.cmp(integer2),
            (Item::Integer(_), Item::List(list2)) => List(vec![self.clone()]).cmp(list2),
            (Item::List(list1), Item::Integer(_)) => list1.cmp(&List(vec![other.clone()])),
            (Item::List(list1), Item::List(list2)) => list1.cmp(list2),
        }
    }
}

impl PartialOrd for List {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for List {
    fn cmp(&self, other: &Self) -> Ordering {
        let mut iter1 = self.0.iter();
        let mut iter2 = other.0.iter();
        loop {
            match (iter1.next(), iter2.next()) {
                (None, None) => return Ordering::Equal,
                (None, Some(_)) => return Ordering::Less,
                (Some(_), None) => return Ordering::Greater,
                (Some(item1), Some(item2)) => {
                    let result = item1.cmp(item2);
                    if result != Ordering::Equal {
                        return result;
                    }
                }
            }
        }
    }
}

fn parse_packet_pairs(text: &str) -> Option<Vec<(List, List)>> {
    text.split("\n\n").map(parse_packet_pair).collect()
}

fn parse_packet_pair(text: &str) -> Option<(List, List)> {
    let (packet_text1, packet_text2) = text.trim().split('\n').collect_tuple()?;
    let packet1 = List::parse(packet_text1)?;
    let packet2 = List::parse(packet_text2)?;
    Some((packet1, packet2))
}

impl List {
    fn parse(text: &str) -> Option<Self> {
        type Chars<'a> = Peekable<std::str::Chars<'a>>;
        const BASE: u32 = 10;

        fn parse_item(chars: &mut Chars) -> Option<Item> {
            match chars.peek()? {
                '[' => parse_list(chars).map(Item::List),
                '0'..='9' => parse_integer(chars).map(Item::Integer),
                _ => None,
            }
        }

        fn parse_list(chars: &mut Chars) -> Option<List> {
            if chars.next()? != '[' {
                return None;
            }
            let mut items = Vec::new();
            let mut first = true;
            while *chars.peek()? != ']' {
                if first {
                    first = false
                } else if chars.next()? != ',' {
                    return None;
                }
                items.push(parse_item(chars)?);
            }
            // Consume ']'
            chars.next()?;
            Some(List(items))
        }

        fn parse_integer(chars: &mut Chars) -> Option<Integer> {
            let mut digits = Vec::new();
            while let Some(c) = chars.peek() {
                let Some(digit) = c.to_digit(BASE) else {
                    break;
                };
                digits.push(digit);
                // Consume digit
                chars.next()?;
            }
            Some(Integer(digits_to_int(&digits, BASE)))
        }

        parse_list(&mut text.chars().peekable())
    }
}

fn digits_to_int(digits: &[u32], base: u32) -> u32 {
    digits.iter().fold(0, |result, digit| result * base + digit)
}

fn index_of<T: PartialEq>(slice: &[T], element: &T) -> Option<usize> {
    slice.iter().position(|e| e == element)
}

impl Display for Item {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Item::List(list) => list.fmt(f),
            Item::Integer(integer) => integer.fmt(f),
        }
    }
}

impl Display for List {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "[")?;
        let mut first = true;
        for item in self.0.iter() {
            if first {
                first = false;
            } else {
                write!(f, ",")?;
            }
            item.fmt(f)?;
        }
        write!(f, "]")
    }
}

impl Display for Integer {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse() {
        fn i(integer: u32) -> Item {
            Item::Integer(Integer(integer))
        }
        // I don't want to use "l", so use "b" for...bracket?
        fn b(items: &[Item]) -> Item {
            Item::List(List(items.to_vec()))
        }

        let actual = List::parse("[1,1,3,1,1]").unwrap();
        let expected = List(vec![i(1), i(1), i(3), i(1), i(1)]);
        assert_eq!(expected, actual);

        let actual = List::parse("[[1],[2,3,4]]").unwrap();
        let expected = List(vec![b(&[i(1)]), b(&[i(2), i(3), i(4)])]);
        assert_eq!(expected, actual);

        let actual = List::parse("[9]").unwrap();
        let expected = List(vec![i(9)]);
        assert_eq!(expected, actual);

        let actual = List::parse("[[4,4],4,4]").unwrap();
        let expected = List(vec![b(&[i(4), i(4)]), i(4), i(4)]);
        assert_eq!(expected, actual);

        let actual = List::parse("[]").unwrap();
        let expected = List(vec![]);
        assert_eq!(expected, actual);

        let actual = List::parse("[[[]]]").unwrap();
        let expected = List(vec![b(&[b(&[])])]);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_examples_part1() {
        let (list1, list2) = parse_packet_pair("[1,1,3,1,1]\n[1,1,5,1,1]").unwrap();
        assert!(List::is_ordered(&list1, &list2));

        let (list1, list2) = parse_packet_pair("[[1],[2,3,4]]\n[[1],4]").unwrap();
        assert!(List::is_ordered(&list1, &list2));

        let (list1, list2) = parse_packet_pair("[9]\n[[8,7,6]]").unwrap();
        assert!(!List::is_ordered(&list1, &list2));

        let (list1, list2) = parse_packet_pair("[[4,4],4,4]\n[[4,4],4,4,4]").unwrap();
        assert!(List::is_ordered(&list1, &list2));

        let (list1, list2) = parse_packet_pair("[7,7,7,7]\n[7,7,7]").unwrap();
        assert!(!List::is_ordered(&list1, &list2));

        let (list1, list2) = parse_packet_pair("[]\n[3]").unwrap();
        assert!(List::is_ordered(&list1, &list2));

        let (list1, list2) = parse_packet_pair("[[[]]]\n[[]]").unwrap();
        assert!(!List::is_ordered(&list1, &list2));

        let (list1, list2) =
            parse_packet_pair("[1,[2,[3,[4,[5,6,7]]]],8,9]\n[1,[2,[3,[4,[5,6,0]]]],8,9]").unwrap();
        assert!(!List::is_ordered(&list1, &list2));

        assert_eq!(13, Day13.part1(Example::Example, Debug::NotDebug));
    }

    #[test]
    fn test_real_part1() {
        assert_eq!(6656, Day13.part1(Example::Real, Debug::NotDebug));
    }

    #[test]
    fn test_examples_part2() {
        assert_eq!(140, Day13.part2(Example::Example, Debug::NotDebug));
    }

    #[test]
    fn test_real_part2() {
        assert_eq!(19716, Day13.part2(Example::Real, Debug::NotDebug));
    }
}
