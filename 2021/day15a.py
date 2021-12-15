#!/usr/bin/env python3

INPUT_FILE = 'input15.txt'

import unittest
import math

DEBUG = True
def debug_print(*args, **kwargs):
    if DEBUG:
        print(*args, **kwargs)

def main():
    if 'INPUT_FILE' not in globals():
        unittest.main()
        return
    risks = parse_text(get_text(INPUT_FILE))
    cost = find_lowest_cost(risks)
    print(cost)

def find_lowest_cost(risks):
    costs = [[math.inf] * len(row) for row in risks]
    costs[0][0] = 0
    seen = set()
    num_cells = sum(len(row) for row in risks)
    while len(seen) < num_cells:
        index = get_lowest_cost_index_not_seen(costs, seen)
        cost = at(costs, index)
        for neighbor_index in get_neighbor_indices(risks, index):
            new_cost = cost + at(risks, neighbor_index)
            if at(costs, neighbor_index) > new_cost:
                set_value(costs, neighbor_index, new_cost)
        seen.add(index)
    # print('\n'.join(''.join(f'{cost: 3}' for cost in row) for row in costs))
    return costs[-1][-1]

def get_lowest_cost_index_not_seen(costs, seen):
    cost_func = lambda index: at(costs, index)
    return min((i for i in get_indices(costs) if i not in seen),
               key=cost_func)

def set_value(grid, index, value):
    row, col = index
    grid[row][col] = value

def at(grid, index):
    row, col = index
    return grid[row][col]

NEIGHBOR_DIFFS = [
    (-1, +0),
    (+0, -1),
    (+0, +1),
    (+1, +0),
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
    return [[int(w) for w in line.strip()]
            for line in text.strip().split('\n')]

def get_text(filename):
    with open(filename) as f:
        return f.read()

EXAMPLE_TEXT = '''
1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581
'''
class Test(unittest.TestCase):
    def test_example(self):
        grid = parse_text(EXAMPLE_TEXT)
        cost = find_lowest_cost(grid)
        self.assertEqual(cost, 40)

if __name__ == '__main__':
    main()
