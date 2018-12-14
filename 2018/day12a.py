#!/usr/bin/env python3

import sys
import re
from collections import defaultdict

TO_LEFT, TO_RIGHT = 2, 2

def solve(initial_state, mapping, num_generations):
    state = initial_state
    for _ in range(num_generations):
        state = next_state(state, mapping)
    pots_with_flowers = get_plant_indices(state)
    # print(pots_with_flowers)
    return sum(pots_with_flowers)

def next_state(state, mapping):
    plant_indices = get_plant_indices(state)
    smallest, largest = min(plant_indices), max(plant_indices)
    smallest_possible = smallest - 2
    largest_possible = largest + 2
    indices = range(smallest_possible, largest_possible + 1)
    return defaultdict(bool, {i: next_pot(state, mapping, i)
                              for i in indices})

def next_pot(state, mapping, index):
    mini_state = get_mini_state(state, index)
    return mapping[mini_state]

def get_mini_state(state, index):
    indices = range(index - TO_LEFT, index + TO_RIGHT + 1)
    return tuple(state[i] for i in indices)

def get_plant_indices(state):
    return {i for i, b in state.items() if b}

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
    num_generations = 20
    initial_state, mapping = get_input()
    # print(initial_state)
    # print(mapping)
    print(solve(initial_state, mapping, num_generations))

if __name__ == '__main__':
    main()
