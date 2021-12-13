#!/usr/bin/env python3

INPUT_FILE = 'input13.txt'

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
    data = parse_lines(lines)
    _, score = find_optimal_arrangment(data)
    print(score)

def find_optimal_arrangment(data):
    people = list(set(person for person, _, _ in data))
    score_dict = {(person, neighbor): score
                  for person, neighbor, score in data}
    score_arrangement_func = create_score_arrangment_func(score_dict)
    optimal_arrangement = max(get_permutations(people),
                              key=score_arrangement_func)
    return optimal_arrangement, score_arrangement_func(optimal_arrangement)

def create_score_arrangment_func(score_dict):
    return lambda arrangment: sum(score_dict[pair]
                                  for pair in get_pairs(arrangment))

def get_pairs(lst):
    yield from get_forward_pairs(lst)
    yield from get_backward_pairs(lst)

def get_forward_pairs(lst):
    for e1, e2 in windows(lst, 2):
        yield e1, e2
    yield lst[-1], lst[0]

def get_backward_pairs(lst):
    for e1, e2 in windows(list(reversed(lst)), 2):
        yield e1, e2
    yield lst[0], lst[-1]

def windows(lst, window_length):
    for i in range(len(lst) - window_length + 1):
        yield lst[i:i + window_length]

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

LINE_RE = re.compile(r'(\w+) would (gain|lose) (\d+) happiness units by sitting next to (\w+)\.')
def parse_line(line):
    try:
        person, gain_lose, score, neighbor = LINE_RE.match(line).groups()
    except Exception as e:
        print(f'Could not parse line: "{line}"')
        raise e
    score = int(score)
    if gain_lose == 'lose':
        score = -score
    return (person, neighbor, score)

def get_lines(filename):
    with open(filename) as f:
        return [line.strip() for line in f.readlines()]

def get_shifted_versions(lst):
    return [lst[i:] + lst[:i] for i in range(len(lst))]

EXAMPLE_LINES = [
    'Alice would gain 54 happiness units by sitting next to Bob.',
    'Alice would lose 79 happiness units by sitting next to Carol.',
    'Alice would lose 2 happiness units by sitting next to David.',

    'Bob would gain 83 happiness units by sitting next to Alice.',
    'Bob would lose 7 happiness units by sitting next to Carol.',
    'Bob would lose 63 happiness units by sitting next to David.',

    'Carol would lose 62 happiness units by sitting next to Alice.',
    'Carol would gain 60 happiness units by sitting next to Bob.',
    'Carol would gain 55 happiness units by sitting next to David.',

    'David would gain 46 happiness units by sitting next to Alice.',
    'David would lose 7 happiness units by sitting next to Bob.',
    'David would gain 41 happiness units by sitting next to Carol.',
]
OPTIMAL_ARRANGEMENT = ['Bob', 'Alice', 'David', 'Carol']
POSSIBLE_OPTIMAL_ARRANGEMENTS = \
    get_shifted_versions(OPTIMAL_ARRANGEMENT) + \
    get_shifted_versions(list(reversed(OPTIMAL_ARRANGEMENT)))
class Test(unittest.TestCase):
    def test_get_shifted_versions(self):
        lst = [1, 2, 3]
        shifted = [
            [1, 2, 3],
            [2, 3, 1],
            [3, 1, 2],
        ]
        self.assertEqual(get_shifted_versions(lst), shifted)

    def test_examples(self):
        data = parse_lines(EXAMPLE_LINES)
        arrangement, score = find_optimal_arrangment(data)
        self.assertIn(arrangement, POSSIBLE_OPTIMAL_ARRANGEMENTS)
        self.assertEqual(score, 330)

if __name__ == '__main__':
    main()
