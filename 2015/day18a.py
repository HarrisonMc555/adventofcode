#!/usr/bin/env python3

# INPUT_FILE = 'input18.txt'

import unittest
import math

DEBUG = True
def debug_print(*args, **kwargs):
    if DEBUG:
        print(*args, **kwargs)

NUM_STEPS = 100
def main():
    if 'INPUT_FILE' not in globals():
        unittest.main()
        return
    grid = parse_text(get_text(INPUT_FILE))
    for _ in range(NUM_STEPS):
        grid = step_grid(grid)
    print(count_grid(grid))

def count_grid(grid):
    return sum(1 for row in grid for cell in row if cell)

def step_grid(grid):
    new_grid = [[False] * len(grid[0]) for _ in range(len(grid))]
    for row, col in get_indices(grid):
        new_grid[row][col] = next_state(grid, row, col)
    return new_grid

def next_state(grid, row, col):
    num_neighbors = count_neighbors(grid, row, col)
    if grid[row][col]:
        return num_neighbors == 2 or num_neighbors == 3
    else:
        return num_neighbors == 3

def count_neighbors(grid, row, col):
    return sum(1 for nr, nc in get_neighbor_indices(grid, (row, col))
               if grid[nr][nc])

NEIGHBOR_DIFFS = [
    (-1, -1),
    (-1, +0),
    (-1, +1),
    (+0, -1),
    # (+0, +0),
    (+0, +1),
    (+1, -1),
    (+1, +0),
    (+1, +1),
]
def get_neighbor_indices(grid, index):
    row, col = index
    neighbor_indices = [(row + dr, col + dc) for dr, dc in NEIGHBOR_DIFFS]
    return [i for i in neighbor_indices if in_range(grid, i)]

def in_range(grid, index):
    row, col = index
    return row in range(len(grid)) and col in range(len(grid[0]))

def get_indices(grid):
    return [(r, c) for r in range(len(grid)) for c in range(len(grid[0]))]

def parse_text(text):
    return [parse_line(line) for line in text.strip().split('\n')]

def parse_line(line):
    return [parse_cell(c) for c in line]

def parse_cell(c):
    return c == '#'

def get_text(filename):
    with open(filename) as f:
        return f.read().strip()

EXAMPLE_GRIDS = [
    '''
.#.#.#
...##.
#....#
..#...
#.#..#
####..
''',
    '''
..##..
..##.#
...##.
......
#.....
#.##..
''',
    '''
..###.
......
..###.
......
.#....
.#....
''',
    '''
...#..
......
...#..
..##..
......
......
''',
    '''
......
......
..##..
..##..
......
......
''',
]
class Test(unittest.TestCase):
    def test_examples(self):
        grid = parse_text(EXAMPLE_GRIDS[0])
        for next_grid_string in EXAMPLE_GRIDS[1:]:
            grid = step_grid(grid)
            self.assertEqual(grid, parse_text(next_grid_string))
        self.assertEqual(count_grid(grid), 4)

if __name__ == '__main__':
    main()
