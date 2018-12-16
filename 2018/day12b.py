#!/usr/bin/env python3
#pylint: disable=invalid-name, too-few-public-methods

import sys
import re
from collections import defaultdict

class BiTree:
    def __init__(self, left, mid, right):
        self.left = left
        self.mid = mid
        self.right = right

def solve(initial_state, mapping, num_generations):
    return sum(initial_state)

def to_bool(char):
    return char == '#'

def get_input():
    initial_state_line = input()
    input() # blank line
    notes_lines = [line.strip() for line in sys.stdin.readlines()]

    initial_state = parse_initial_state(initial_state_line)
    notes = [parse_line(line) for line in notes_lines]

    # mapping = dict(notes)
    mapping = defaultdict(bool, notes)
    return initial_state, mapping

PATTERN_INITIAL_STATE = re.compile(r'initial state: ([#.]+)')
def parse_initial_state(line):
    string = PATTERN_INITIAL_STATE.match(line).groups()[0]
    return defaultdict(bool, {i: c == '#' for i, c in enumerate(string)})

PATTERN_LINE = re.compile(r'([#.]{5}) => ([#.])')
def parse_line(line):
    mini_state, result = PATTERN_LINE.match(line).groups()
    mini_state = tuple(to_bool(c) for c in mini_state)
    result = to_bool(result)
    return mini_state, result

def main():
    # num_generations = 50000000000
    num_generations = 20
    initial_state, mapping = get_input()
    print(solve(initial_state, mapping, num_generations))

if __name__ == '__main__':
    main()
