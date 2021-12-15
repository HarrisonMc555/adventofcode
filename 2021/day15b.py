#!/usr/bin/env python3

INPUT_FILE = 'input15.txt'

import unittest
import itertools
import math
from collections import defaultdict
from heapq import *

DEBUG = True
def debug_print(*args, **kwargs):
    if DEBUG:
        print(*args, **kwargs)

def main():
    if 'INPUT_FILE' not in globals():
        unittest.main()
        return
    small_grid = parse_text(get_text(INPUT_FILE))
    large_grid = expand_grid(small_grid)
    cost = a_star(large_grid)
    print(cost)

def a_star(grid):
    num_rows = len(grid)
    num_cols = len(grid[0])
    def heuristic(index):
        row, col = index
        return abs(num_rows - row) + abs(num_cols - col)

    start_index = 0, 0
    start_cost = 0

    cost_from_start = defaultdict(lambda: math.inf)
    cost_from_start[start_index] = start_cost

    goal = (num_rows - 1, num_cols - 1)

    # Visited but not expanded
    # open_list = {start_index}
    open_list = PriorityQueue()
    open_list.add(start_index, start_cost + heuristic(start_index))

    while open_list:
        index = open_list.pop()
        if index == goal:
            return cost_from_start[goal]
        cost = cost_from_start[index]
        for neighbor_index in get_neighbor_indices(grid, index):
            additional_cost = at(grid, neighbor_index)
            # row, col = neighbor_index
            # additional_cost = grid[row][col]
            new_cost = cost + additional_cost
            if new_cost < cost_from_start[neighbor_index]:
                cost_from_start[neighbor_index] = new_cost
                estimated_cost = new_cost + heuristic(neighbor_index)
                open_list.add(neighbor_index, estimated_cost)
    raise Exception('no path to goal')

# def find_lowest_cost(risks):
#     pq = PriorityQueue()
#     start_index = 0
#     start_cost = 0
#     pq.add(start_index, start_cost)
#     costs = defaultdict(lambda: math.inf)
#     # costs = [[math.inf] * len(row) for row in risks]
#     costs[(0, 0)] = 0
#     # num_cells = sum(len(row) for row in risks)
#     while pq:
#         index = heappop(pq)
#         # cost = at(costs, index)
#         cost = costs[index]
#         for neighbor_index in get_neighbor_indices(risks, index):
#             # new_cost = cost + at(risks, neighbor_index)
#             new_cost = cost + at(risks, neighbor_index)
#             if at(costs, neighbor_index) > new_cost:
#                 set_value(costs, neighbor_index, new_cost)
#     while len(seen) < num_cells:
#         while
#         index = get_lowest_cost_index_not_seen(costs, seen)
#         cost = at(costs, index)
#         for neighbor_index in get_neighbor_indices(risks, index):
#             new_cost = cost + at(risks, neighbor_index)
#             if at(costs, neighbor_index) > new_cost:
#                 set_value(costs, neighbor_index, new_cost)
#         seen.add(index)
#     # print('\n'.join(''.join(f'{cost: 3}' for cost in row) for row in costs))
#     return costs[-1][-1]

# def get_lowest_cost_index_not_seen(costs, seen):
#     cost_func = lambda index: at(costs, index)
#     return min((i for i in get_indices(costs) if i not in seen),
#                key=cost_func)

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

NUM_EXPANDED_ROWS = 5
NUM_EXPANDED_COLS = 5
def expand_grid(grid):
    minor_rows = len(grid)
    minor_cols = len(grid[0])
    new_rows = minor_rows * NUM_EXPANDED_ROWS
    new_cols = minor_cols * NUM_EXPANDED_COLS
    new_grid = [[0] * new_cols for _ in range(new_rows)]
    for major_row in range(NUM_EXPANDED_COLS):
        for major_col in range(NUM_EXPANDED_ROWS):
            for minor_row in range(minor_rows):
                new_row = major_row * minor_rows + minor_row
                for minor_col in range(minor_cols):
                    new_col = major_col * minor_cols + minor_col
                    orig_value = grid[minor_row][minor_col]
                    new_value = orig_value + major_row + major_col
                    while new_value > 9:
                        new_value -= 9
                    new_grid[new_row][new_col] = new_value
    return new_grid

def parse_text(text):
    return [[int(w) for w in line.strip()]
            for line in text.strip().split('\n')]

def get_text(filename):
    with open(filename) as f:
        return f.read()

class PriorityQueue:
    def __init__(self):
        self.queue = []
        self.entry_finder = {}
        self.counter = itertools.count()

    def add(self, item, priority=0):
        if item in self.entry_finder:
            self.remove(item)
        count = next(self.counter)
        entry = [priority, count, True, item]
        self.entry_finder[item] = entry
        heappush(self.queue, entry)

    def remove(self, item):
        entry = self.entry_finder[item]
        entry[2] = False

    def pop(self):
        while self.queue:
            priority, count, is_valid, item = heappop(self.queue)
            if is_valid:
                del self.entry_finder[item]
                return item
        raise KeyError('pop from an empty PriorityQueue')

    def __bool__(self):
        return any(entry[2] for entry in self.queue)

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
    def test_example_part1(self):
        grid = parse_text(EXAMPLE_TEXT)
        cost = a_star(grid)
        self.assertEqual(cost, 40)

    def test_example_part2(self):
        small_grid = parse_text(EXAMPLE_TEXT)
        large_grid = expand_grid(small_grid)
        cost = a_star(large_grid)
        self.assertEqual(cost, 315)

if __name__ == '__main__':
    main()
