#!/usr/bin/env python3

INPUT_FILE = 'input09.txt'

import unittest
import re

DEBUG = False
def debug_print(*args, **kwargs):
    if DEBUG:
        print(*args, **kwargs)

def main():
    if 'INPUT_FILE' not in globals():
        unittest.main()
        return
    lines = get_lines(INPUT_FILE)
    route, cost = run(lines)
    print(cost)

def run(lines):
    parsed_lines = parse_lines(lines)
    locs = list(set([from_loc for from_loc, _, _ in parsed_lines] +
                    [to_loc for _, to_loc, _ in parsed_lines]))
    debug_print(locs)
    route_cost_func = create_route_cost_func(parsed_lines)
    routes = get_permutations(locs)
    route = min(routes, key=route_cost_func)
    return route, route_cost_func(route)

def create_route_cost_func(parsed_lines):
    pair_to_length = {}
    for from_loc, to_loc, cost in parsed_lines:
        pair_to_length[(from_loc, to_loc)] = cost
        pair_to_length[(to_loc, from_loc)] = cost
    debug_print(pair_to_length)
    return lambda route: sum(pair_to_length[pair]
                             for pair in zip(route, route[1:]))

def get_permutations(choices):
    if not choices:
        yield []
        return
    value = choices[0]
    permutations = list(get_permutations(choices[1:]))
    for permutation in permutations:
        for i in range(len(permutation) + 1):
            yield permutation[:i] + [value] + permutation[i:]

def parse_lines(lines):
    return [parse_line(line) for line in lines]

LINE_RE = re.compile(r'(\w+) to (\w+) = (\d+)')
def parse_line(line):
    from_loc, to_loc, cost_s = LINE_RE.match(line).groups()
    return (from_loc, to_loc, int(cost_s))

def get_lines(filename):
    with open(filename) as f:
        return [line.strip() for line in f.readlines()]

TEST_LINES = [
    "London to Dublin = 464",
    "London to Belfast = 518",
    "Dublin to Belfast = 141",    
]
class Test(unittest.TestCase):
    def test_permutations(self):
        permutations = set(tuple(p) for p in get_permutations([1, 2, 3]))
        expected = {
            (1, 2, 3),
            (1, 3, 2),
            (2, 1, 3),
            (2, 3, 1),
            (3, 1, 2),
            (3, 2, 1),
        }
        self.assertEqual(permutations, expected)

    def test_example(self):
        route, cost = run(TEST_LINES)
        expected_routes = [
            ['London', 'Dublin', 'Belfast'],
            ['Belfast', 'Dublin', 'London'],
        ]
        self.assertIn(route, expected_routes)
        self.assertEqual(cost, 605)

if __name__ == '__main__':
    main()
