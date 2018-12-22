#!/usr/bin/env python3

import sys
import re
from collections import defaultdict

TO_LEFT, TO_RIGHT = 2, 2
NUM_GENERATIONS = 50000000000
LOOP_LENGTH = 2**3 * 3**2 * 5 * 7

def solve(initial_state, mapping):
    state = initial_state
    pattern_to_index_offset = {}
    index_to_pattern_offset = {}
    for index in range(NUM_GENERATIONS):
        plant_indices = get_plant_indices(state)
        pattern = get_pattern(plant_indices)
        offset = get_offset(plant_indices)
        answer = find_answer(pattern_to_index_offset, index_to_pattern_offset,
                             pattern, offset, index)
        if answer is not None:
            return answer
        index_to_pattern_offset[index] = pattern, offset
        pattern_to_index_offset[pattern] = index, offset
        state = next_state(state, mapping)
    pots_with_flowers = get_plant_indices(state)
    return sum(pots_with_flowers)

def find_answer(pattern_to_index_offset, index_to_pattern_offset, pattern,
                offset, index):
    result = find_cycle(pattern_to_index_offset, pattern, index, offset)
    if result is None:
        return None
    answer_index, cycle_offset, num_cycles = result
    pattern, offset = index_to_pattern_offset[answer_index]
    return calculate_answer(pattern, offset, num_cycles, cycle_offset)

def get_pattern(plant_indices):
    plant_indices = sorted(plant_indices)
    min_index = plant_indices[0]
    return tuple(plant_index - min_index for plant_index in plant_indices)

def get_offset(plant_indices):
    return min(plant_indices)

def find_cycle(pattern_to_index_offset, pattern, index, offset):
    if pattern not in pattern_to_index_offset:
        return None
    start_index, first_offset = pattern_to_index_offset[pattern]
    end_index = index
    cycle_length = end_index - start_index
    answer_cycle_index = (NUM_GENERATIONS - start_index) % cycle_length
    answer_absolute_index = start_index + answer_cycle_index
    num_cycles = (NUM_GENERATIONS - start_index) // cycle_length
    cycle_offset = offset - first_offset
    return answer_absolute_index, cycle_offset, num_cycles

def calculate_answer(pattern, pattern_offset, num_cycles, cycle_offset):
    offset = pattern_offset + num_cycles*cycle_offset
    return sum(offset + pot_offset for pot_offset in pattern)

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
    initial_state, mapping = get_input()
    print(solve(initial_state, mapping))

if __name__ == '__main__':
    main()
