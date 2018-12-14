#!/usr/bin/env python3
#pylint: disable=invalid-name, too-few-public-methods

import sys
import re
from collections import defaultdict

def solve2(initial_state, mapping, num_generations):
    state = initial_state
    mappings = defaultdict(dict, {5: mapping})
    # for _ in range(num_generations):
    for i in range(num_generations):
        if i % 10000 == 0:
            print('{} ({:.2f})'.format(i, i / num_generations))
        # mutable mappings
        state = get_next_state2(state, mappings)
    # print(mappings)
    return sum(get_plant_indices(state))

def get_next_state2(state, mappings):
    smallest, largest, state_list = get_state_list(state)
    mid_state = get_next_state_from_list(state_list, mappings)
    left_edge, right_edge = get_next_edge_state_from_list(state_list, mappings)
    state_list = left_edge + mid_state + right_edge
    state = defaultdict(bool, zip(range(smallest - 2, largest + 2 + 1),
                                  state_list))
    return state

# mutable mappings
def get_next_state_from_list(state_list, mappings):
    sid = state_list_id(state_list)
    # print(sid, 'calling get_next_state_from_list({})'.format(state_list))
    length = len(state_list)
    assert length >= 5
    # print(length)
    mapping = mappings[length]
    state_tuple = tuple(state_list)
    if length == 5:
        result = [mapping[state_tuple]]
        # print(sid, 'length is 5, so {} ({})'.format(result, type(result)))
        return result
        # return [mapping[state_tuple]]
    if state_tuple in mapping:
        result = mapping[state_tuple]
        # print(sid, 'found {} ({})'.format(result, type(result)))
        return result
        # return mapping[state_tuple]
    calc_length = length - 4
    first_length, second_length = calc_split_lengths(calc_length)
    first = state_list[:first_length]
    second = state_list[-second_length:]
    # print(sid, '=== split up time! ===')
    # print(sid, '\tstate_list:', state_list)
    # print(sid, '\tfirst:', first)
    # print(sid, '\tsecond:', second)
    next_first = get_next_state_from_list(first, mappings)
    # print(sid, 'type(next_first):', type(next_first))
    next_second = get_next_state_from_list(second, mappings)
    # print(sid, 'type(next_second):', type(next_second))
    # print(sid, '=== next assertion time! ===')
    # print(sid, '\tstate_list:', state_list)
    # print(sid, '\tfirst:', first)
    # print(sid, '\tsecond:', second)
    # print(sid, '\tnext_first:', next_first)
    # print(sid, '\tnext_second:', next_second)
    # print(sid, '\tlength:', length)
    # print(sid, '\tlen(first)', len(first))
    # print(sid, '\tlen(next_first)', len(next_first))
    assert len(first) - 4 == len(next_first)
    # print(sid, '\tlen(second)', len(second))
    # print(sid, '\tlen(next_second)', len(next_second))
    assert len(second) - 4 == len(next_second)

    length_from_next_second = calc_length - len(next_first)
    from_next_second = next_second[-length_from_next_second:] \
                       if length_from_next_second > 0 else []
    next_state = next_first[:calc_length] + from_next_second
    # print(sid, '=== normal assertion time! ===')
    # print(sid, 'state_list:', state_list)
    # print(sid, 'next_first:', next_first)
    # print(sid, 'next_second:', next_second)
    # print(sid, 'length:', length)
    # print(sid, 'calc_length:', calc_length)
    # print(sid, 'length_from_next_second:', length_from_next_second)
    # print(sid, 'next_state:', next_state)
    # print(sid, 'len(next_state)', len(next_state))
    assert len(next_state) == calc_length
    mapping[state_tuple] = next_state
    return next_state

def state_list_id(state_list):
    return ''.join(to_char(b) for b in state_list)

def to_char(bool_):
    return '#' if bool_ else '.'

def calc_split_lengths(length):
    assert length > 1
    # give second the rounded down version
    second_calc_length = length // 2
    first_calc_length = length - second_calc_length
    first_need_length = max(first_calc_length + 4, 5)
    second_need_length = max(second_calc_length + 4, 5)
    return first_need_length, second_need_length

def get_next_edge_state_from_list(state_list, mappings):
    # print('calling get_next_edge_state_from_list({})'.format(state_list))
    left_edge = [False] * 4 + state_list[:4]
    # print('left_edge:', left_edge)
    right_edge = state_list[-4:] + [False] * 4
    # print('right_edge:', right_edge)
    next_left = get_next_state_from_list(left_edge, mappings)
    # print('next_left:', next_left)
    next_right = get_next_state_from_list(right_edge, mappings)
    # print('next_right:', next_right)

    # print('=== edge assertion time! ===')
    # print('left_edge:', left_edge)
    # print('right_edge:', right_edge)
    # print('state_list:', state_list)
    # print('next_left:', next_left)
    assert len(next_left) == 4
    # print('next_right:', next_right)
    assert len(next_right) == 4
    return next_left, next_right

def get_state_list(state):
    plant_indices = get_plant_indices(state)
    smallest, largest = min(plant_indices), max(plant_indices)
    lst = [i in plant_indices for i in range(smallest, largest + 1)]
    return smallest, largest, lst

TO_LEFT, TO_RIGHT = 2, 2

def solve(initial_state, mapping, num_generations):
    state = initial_state
    cache = {}
    partials = [sum(get_plant_indices(state))]
    for i in range(num_generations):
        if i % 1000 == 0:
            print('{: 8d} ({})'.format(i, len(cache)))
        state_tuple = get_state_tuple(state)
        #pylint: disable=consider-using-get
        if state_tuple in cache:
            print('duplicate')
            next_state = cache[state_tuple]
        else:
            next_state = get_next_state(state, mapping)
            cache[state_tuple] = next_state
        state = next_state
        partials.append(sum(get_plant_indices(state)))
    pots_with_flowers = get_plant_indices(state)
    # print(pots_with_flowers)
    return sum(pots_with_flowers)

def get_next_state(state, mapping):
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

def get_state_tuple(state):
    return tuple(sorted(get_plant_indices(state)))

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
    num_generations = 50000000000
    # num_generations = 100000
    initial_state, mapping = get_input()
    # print(solve(initial_state, mapping, num_generations))
    print(solve2(initial_state, mapping, num_generations))

if __name__ == '__main__':
    main()
