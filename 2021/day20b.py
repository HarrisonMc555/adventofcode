#!/usr/bin/env python3

INPUT_FILE = 'input20.txt'
# INPUT_FILE = 'example20.txt'

import unittest
import re

DEBUG = False
# DEBUG = True
def debug_print(*args, **kwargs):
    if DEBUG:
        print(*args, **kwargs)

def main():
    if 'INPUT_FILE' not in globals():
        unittest.main()
        return
    algorithm, state = parse_text(get_text(INPUT_FILE))
    if DEBUG:
        algorithm[0] = True
    print(run(algorithm, state))

NUM_STEPS = 50
def run(algorithm, state):
    if algorithm[0]:
        return run_flipping(algorithm, state)
    else:
        return run_normal(algorithm, state)

def run_normal(algorithm, state):
    debug_print(state_to_string(state))
    debug_print()
    for _ in range(NUM_STEPS):
        state = run_step_normal(algorithm, state)
        debug_print(state_to_string(state))
        debug_print()
    return len(state)

def run_step_normal(algorithm, state):
    new_state = set()
    coords_to_consider = {neighbor_coord for coord in state
                          for neighbor_coord in get_neighbor_coords(coord)}
    return {coord for coord in coords_to_consider
            if run_step_coord_normal(algorithm, state, coord)}

def run_flipping(algorithm, state):
    if NUM_STEPS % 2 != 0 and algorithm[0]:
        import math
        return math.inf
    debug_print(state_to_string(state))
    debug_print()
    for step in range(NUM_STEPS):
        flipped_to_non_flipped = step % 2 == 0
        state = run_step_flipping(algorithm, state, flipped_to_non_flipped)
        debug_print(state_to_string(state, flipped_to_non_flipped))
        debug_print()
    return len(state)

def run_step_coord_normal(algorithm, state, coord):
    index = get_index(state, coord)
    return algorithm[index]

def run_step_flipping(algorithm, state, flipped_to_non_flipped):
    new_state = set()
    coords_to_consider = {neighbor_coord for coord in state
                          for neighbor_coord in get_neighbor_coords(coord)}
    return {coord for coord in coords_to_consider
            if run_step_coord_flipping(algorithm, state, coord, flipped_to_non_flipped)}

def run_step_coord_flipping(algorithm, state, coord, flipped_to_non_flipped):
    coords_flipped = not flipped_to_non_flipped
    index = get_index(state, coord, coords_flipped)
    flip_return_value = flipped_to_non_flipped
    return flip_return_value ^ algorithm[index]

def get_index(state, coord, coords_flipped=False):
    row, col = coord
    bools = [(neighbor_coord in state) ^ coords_flipped
             for neighbor_coord in get_neighbor_coords(coord)]
    return bools_to_index(bools)

######################################################################

def get_neighbor_coords(coord):
    row, col = coord
    return [(r, c) for r in range(row - 1, row + 2)
            for c in range(col - 1, col + 2)]

def bools_to_index(bools):
    result = 0
    for b in bools:
        result *= 2
        if b:
            result += 1
    return result

def parse_text(text):
    algorithm_group, image_group = text.split('\n\n')
    algorithm = [c == '#' for c in algorithm_group.strip()]
    image = [[c == '#' for c in row.strip()]
             for row in image_group.split('\n')]
    state = {(r, c) for r in range(len(image)) for c in range(len(image[r]))
             if image[r][c]}
    return algorithm, state

def get_text(filename):
    with open(filename) as f:
        return f.read().strip()

def state_to_string(state, flipped=False):
    # min_row = min(r for r, _ in state)
    # max_row = max(r for r, _ in state)
    # min_col = min(c for c, _ in state)
    # max_col = max(c for c, _ in state)
    min_row, min_col = -2, -2
    max_row, max_col = 7, 7
    return '\n'.join(''.join('#' if ((r, c) in state) ^ flipped else '.'
                             for c in range(min_col, max_col + 1))
                     for r in range(min_row, max_row + 1))
                     #         for c in range(min_col - 1, max_col + 2))
                     # for r in range(min_row - 1, max_row + 2))

class Test(unittest.TestCase):
    pass

if __name__ == '__main__':
    main()
