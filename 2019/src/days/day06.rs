use std::collections::HashMap;
use std::fmt;
use std::hash::Hash;

const INPUT: &str = include_str!("../../static/day06.txt");

const ROOT_LABEL: &str = "COM";
const YOU_LABEL: &str = "YOU";
const SAN_LABEL: &str = "SAN";

type Error = String;

#[derive(Eq, PartialEq)]
struct Node<T> {
    data: T,
    children: Vec<Node<T>>,
}

pub fn main() {
    let answer1 = solve1(INPUT).unwrap();
    println!("{}", answer1);
    let answer2 = solve2(INPUT).unwrap();
    println!("{}", answer2);
}

fn solve1(input: &str) -> Result<usize, Error> {
    let tree = tree_from_input(input)?;
    Ok(total_num_orbits(&tree))
}

fn solve2(input: &str) -> Result<usize, Error> {
    let tree = tree_from_input(input)?;
    let you_to_san_len = distance_between(&tree, &YOU_LABEL, &SAN_LABEL)
        .ok_or_else(|| "No path from YOU to SAN".to_string())?;
    // We really want distances between their parents, subtract one for each
    let distance_between_orbits = you_to_san_len - 2;
    Ok(distance_between_orbits)
}

fn distance_between<T>(root: &Node<T>, goal1: &T, goal2: &T) -> Option<usize>
where
    T: Eq + fmt::Debug,
{
    let path1 = find_path_to(root, goal1)?;
    let path2 = find_path_to(root, goal2)?;
    let common_len = suffix_len(&path1, &path2);
    let common_to_goal1 = path1.len() - common_len;
    let common_to_goal2 = path2.len() - common_len;
    Some(common_to_goal1 + common_to_goal2)
}

fn find_path_to<'a, T>(root: &'a Node<T>, goal: &T) -> Option<Vec<&'a T>>
where
    T: PartialEq,
{
    if &root.data == goal {
        return Some(vec![&root.data]);
    }
    let mut child_path = root
        .children
        .iter()
        .filter_map(|child| find_path_to(child, goal))
        .next()?;
    child_path.push(&root.data);
    Some(child_path)
}

fn total_num_orbits<T>(root: &Node<T>) -> usize {
    fn total_num_orbits_helper<U>(node: &Node<U>, level: usize) -> usize {
        let children_num = node
            .children
            .iter()
            .map(|child| total_num_orbits_helper(child, level + 1))
            .sum::<usize>();
        level + children_num
    }
    total_num_orbits_helper(root, 0)
}

fn tree_from_input(input: &str) -> Result<Node<&str>, Error> {
    let pairs = parse_input(input)?;
    let mut mapping = construct_mapping(pairs);
    Ok(construct_tree(&mut mapping, ROOT_LABEL))
}

fn construct_tree<T>(mapping: &mut HashMap<T, Vec<T>>, root: T) -> Node<T>
where
    T: Eq + Hash,
{
    let children = mapping.remove(&root).unwrap_or_else(|| Vec::new());
    let children_nodes = children
        .into_iter()
        .map(|child: T| construct_tree::<T>(mapping, child))
        .collect::<Vec<Node<T>>>();
    Node::new(root, children_nodes)
}

fn construct_mapping<T>(pairs: Vec<(T, T)>) -> HashMap<T, Vec<T>>
where
    T: Eq + Hash,
{
    let mut mapping = HashMap::new();
    for (parent, child) in pairs {
        let children = mapping.entry(parent).or_insert_with(|| Vec::new());
        children.push(child);
    }
    mapping
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

fn suffix_len<T>(xs: &[T], ys: &[T]) -> usize
where
    T: Eq,
{
    xs.iter()
        .rev()
        .zip(ys.iter().rev())
        .take_while(|(x, y)| x == y)
        .count()
}

impl<T> Node<T> {
    fn new(data: T, children: Vec<Node<T>>) -> Self {
        Node { data, children }
    }
}

impl<T> fmt::Debug for Node<T>
where
    T: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let children_strings = self
            .children
            .iter()
            .map(|child| format!("{:?}", child.data))
            .collect::<Vec<_>>()
            .join(", ");
        write!(f, "Node {{ {:?}, [{}] }}", self.data, children_strings)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_suffix_len() {
        let a = [5, 4, 3, 2, 1];
        let b = [999, 9, 3, 2, 1];
        let c = [99, 99, 99, 99, 1];
        let d = [99, 99, 99, 99, 99];
        let e = [99];
        let f: [u32; 0] = [];
        assert_eq!(suffix_len(&a, &b), 3);
        assert_eq!(suffix_len(&a, &c), 1);
        assert_eq!(suffix_len(&a, &d), 0);
        assert_eq!(suffix_len(&a, &e), 0);
        assert_eq!(suffix_len(&a, &f), 0);
    }

    #[test]
    fn answer1() {
        assert_eq!(solve1(INPUT), Ok(308790));
    }

    #[test]
    fn answer2() {
        assert_eq!(solve2(INPUT), Ok(472));
    }
}
