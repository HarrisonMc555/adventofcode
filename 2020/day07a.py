#!/usr/bin/env python3

import re
from collections import defaultdict 

INPUT_FILE = 'input07.txt'
# INPUT_FILE = 'example.txt'
BAG_COLOR = 'shiny gold'

def main():
    lines = get_lines(INPUT_FILE)
    graph = parse_lines(lines)
    reversed_graph = reverse_graph(graph)
    descendants = find_descendants(reversed_graph, BAG_COLOR)
    print(len(descendants))

def get_lines(filename):
    with open(filename) as f:
        return [line.strip() for line in f.readlines() if line]

def parse_lines(lines):
    return dict(parse_line(line) for line in lines)

LINE_RE = re.compile('^(\w+ \w+) bags contain ([a-z0-9, ]+)\.$')
NO_OTHER_BAGS = 'no other bags'
def parse_line(line):
    bag_color, contained_bags = LINE_RE.match(line).groups()
    if contained_bags == NO_OTHER_BAGS:
        return bag_color, []
    contained_bags = parse_contained_bags(contained_bags)
    return bag_color, contained_bags

def parse_contained_bags(text):
    return [parse_contained_bag(word) for word in text.split(', ')]

CONTAINED_BAGS_RE = re.compile('^([0-9]+) (\w+ \w+) bags?$')
def parse_contained_bag(text):
    num, color = CONTAINED_BAGS_RE.match(text).groups()
    return num, color

def reverse_graph(graph):
    result = defaultdict(list)
    for node, edges in graph.items():
        for weight, other_node in edges:
            result[other_node].append((weight, node))
    return result

def find_descendants(graph, node):
    result = set()
    to_process = nodes(graph[node])
    while to_process:
        other_node = to_process.pop()
        if other_node in result:
            continue
        result.add(other_node)
        to_process += nodes(graph[other_node])
    return result

def nodes(items):
    return [n for (w, n) in items]

def weights(items):
    return [w for (w, n) in items]

if __name__ == '__main__':
    main()
