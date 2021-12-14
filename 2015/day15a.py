#!/usr/bin/env python3

INPUT_FILE = 'input15.txt'

import unittest
import re

DEBUG = True
def debug_print(*args, **kwargs):
    if DEBUG:
        print(*args, **kwargs)

NUM_SECONDS = 2503
def main():
    if 'INPUT_FILE' not in globals():
        unittest.main()
        return
    data = parse_lines(get_lines(INPUT_FILE))
    _, score = find_best_proportions(data)
    print(score)

NUM_INGREDIENTS = 100
def find_best_proportions(data):
    proportions = [1] * len(data)
    while sum(proportions) < NUM_INGREDIENTS:
        next_proportions = [add_ingredient(proportions, i)
                            for i in range(len(data))]
        proportions = max(next_proportions,
                          key=lambda ps: score_proportions(data, ps))
    return proportions, score_proportions(data, proportions)

def add_ingredient(proportions, index):
    return [p if i != index else p + 1
            for i, p in enumerate(proportions)]

def score_proportions(data, proportions):
    return product(get_properties(data, proportions))

def get_properties(data, proportions):
    properties = [0] * 4
    for ingredient_index, ingredient in enumerate(data):
        for property_index in range(len(properties)):
            properties[property_index] += ingredient[property_index + 1] * \
                proportions[ingredient_index]
    return [max(p, 0) for p in properties]

def parse_lines(lines):
    return [parse_line(line) for line in lines]

LINE_RE = re.compile(r'(\w+): capacity (-?\d+), durability (-?\d+), flavor (-?\d+), texture (-?\d+), calories (-?\d+)')
def parse_line(line):
    try:
        groups =  LINE_RE.match(line).groups()
        return [groups[0]] + [int(g) for g in groups[1:]]
    except Exception as e:
        print(f'Could not parse line: "{line}"')
        raise e

def get_lines(filename):
    with open(filename) as f:
        return [line.strip() for line in f.readlines()]

def product(xs):
    result = 1
    for x in xs:
        result *= x
    return result

EXAMPLE_LINES = [
    'Butterscotch: capacity -1, durability -2, flavor 6, texture 3, calories 8',
    'Cinnamon: capacity 2, durability 3, flavor -2, texture -1, calories 3',
]
class Test(unittest.TestCase):
    def test_examples(self):
        data = parse_lines(EXAMPLE_LINES)
        proportions, score = find_best_proportions(data)
        self.assertEqual(proportions, [44, 56])
        self.assertEqual(score, 62842880)

if __name__ == '__main__':
    main()
