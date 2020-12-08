#!/usr/bin/env python3

import re
from collections import defaultdict 

INPUT_FILE = 'input07.txt'
# INPUT_FILE = 'example.txt'
BAG_COLOR = 'shiny gold'

def main():
    lines = get_lines(INPUT_FILE)
    graph = parse_lines(lines)
    print(get_total_bags(graph, BAG_COLOR) - 1)

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
    return int(num), color

def get_total_bags(graph, node):
    cached_total_bags = {}
    return get_total_bags_helper(graph, node, cached_total_bags)

def get_total_bags_helper(graph, node, cached_total_bags):
    total_bags = 1 # this bag itself
    for num, other_node in graph[node]:
        if other_node not in cached_total_bags:
            other_total_bags = get_total_bags_helper(
                graph, other_node, cached_total_bags)
            cached_total_bags[other_node] = other_total_bags
        total_bags += num * cached_total_bags[other_node]
    return total_bags

if __name__ == '__main__':
    main()
