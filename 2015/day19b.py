#!/usr/bin/env python3

INPUT_FILE = 'input19.txt'

import unittest
import re

DEBUG = True
def debug_print(*args, **kwargs):
    if DEBUG:
        print(*args, **kwargs)

START = 'e'
def main():
    if 'INPUT_FILE' not in globals():
        unittest.main()
        return
    # rules, goal = parse_text(get_text(INPUT_FILE))
    # print(fewest_steps(rules, goal, START))
    rules, start = parse_text(get_text(INPUT_FILE))
    print(fewest_steps2(rules, start, START))

def fewest_steps2(rules, cur, goal):
    rules.sort(key=lambda rule: len(rule[1]), reverse=True)
    num_steps = 0
    while cur != goal:
        for before, after in rules:
            if after in cur:
                cur = cur.replace(after, before, 1)
                num_steps += 1
                break
        else:
            raise Exception('No rules match')
    return num_steps

def fewest_steps(rules, cur, start):
    possibilities = {start}
    num_steps = 0
    while goal not in possibilities:
        print(f'After {num_steps} there are {len(possibilities)} possibilities')
        num_steps += 1
        new_possibilities = set()
        for text in possibilities:
            new_possibilities.update(get_possibilities(rules, text))
        possibilities = new_possibilities
    return num_steps

def get_possibilities(rules, text):
    possibilities = set()
    for rule in rules:
        possibilities.update(run_rule(rule, text))
    return possibilities

def run_rule(rule, text):
    # pattern, after = rule
    before, after = rule
    pattern = re.compile(before)
    for match in pattern.finditer(text):
        i1, i2 = match.span()
        yield text[:i1] + after + text[i2:]

def parse_text(text):
    rules_text, start = [piece.strip() for piece in text.split('\n\n')]
    return parse_rules(rules_text), start

def parse_rules(text):
    return [parse_rule(line.strip()) for line in text.strip().split('\n')]

def parse_rule(line):
    before, after = line.split(' => ')
    # return re.compile(before), after
    return before, after

def get_text(filename):
    with open(filename) as f:
        return f.read().strip()

EXAMPLE_TEXT = '''
H => HO
H => OH
O => HH

HOH
'''
EXAMPLE_POSSIBILITIES = {
    'HOOH',
    'HOHO',
    'OHOH',
    'HOOH',
    'HHHH',
}
NEW_EXAMPLE_RULES_TEXT = '''
e => H
e => O
H => HO
H => OH
O => HH
'''
GOAL_TO_NUM_STEPS = [
    ('HOH', 3),
    ('HOHOHO', 6),
]
class Test(unittest.TestCase):
    def test_example_part1(self):
        rules, start = parse_text(EXAMPLE_TEXT)
        self.assertEqual(get_possibilities(rules, start),
                         EXAMPLE_POSSIBILITIES)

    def test_example_part2(self):
        rules = parse_rules(NEW_EXAMPLE_RULES_TEXT)
        for goal, num_steps in GOAL_TO_NUM_STEPS:
            self.assertEqual(fewest_steps2(rules, goal, START),
                             num_steps)

if __name__ == '__main__':
    main()
