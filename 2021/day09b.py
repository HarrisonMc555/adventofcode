#!/usr/bin/env python3

INPUT_FILE = 'input09.txt'
# INPUT_FILE = 'example09.txt'

from collections import deque

def main():
    lines = get_lines(INPUT_FILE)
    grid = parse_lines(lines)
    print(run(grid))

def run(grid):
    basins = get_basins(grid)
    three_longest = sorted([len(basin) for basin in basins], reverse=True)[:3]
    return product(three_longest)

def get_basins(grid):
    low_points = get_low_points(grid)
    return [get_basin(grid, low_point) for low_point in low_points]

def get_basin(grid, low_point):
    basin = set()
    basin.add(low_point)
    stack = deque([low_point])
    count = 0
    while stack:
        count += 1
        row_i, col_i = stack.popleft()
        for neighbor_i in get_neighbor_indices(grid, row_i, col_i):
            neighbor_row, neighbor_col = neighbor_i
            if neighbor_i not in basin and \
               grid[neighbor_row][neighbor_col] < 9:
                basin.add(neighbor_i)
                stack.append(neighbor_i)
    return basin

def get_low_points(grid):
    return [(r, c)
            for r in range(len(grid))
            for c in range(len(grid[0]))
            if is_low_point(grid, r, c)
            ]

def is_low_point(grid, row_i, col_i):
    cell = grid[row_i][col_i]
    neighbors = get_neighbors(grid, row_i, col_i)
    return all(cell < neighbor for neighbor in neighbors)

def get_neighbors(grid, row_i, col_i):
    return [grid[r][c] for r, c in get_neighbor_indices(grid, row_i, col_i)]

ROW_COL_DIFFS = [
    (1, 0),
    (-1, 0),
    (0, 1),
    (0, -1),
]
def get_neighbor_indices(grid, row_i, col_i):
    possible_neighbor_indices = [(row_i + row_diff, col_i + col_diff)
                                 for row_diff, col_diff in ROW_COL_DIFFS]
    row_range = range(len(grid))
    col_range = range(len(grid[0]))
    return [(r, c) for r, c in possible_neighbor_indices
            if r in row_range and c in col_range]

def parse_lines(lines):
    return [[int(c) for c in line.strip()] for line in lines]

def parse_line(line):
    unique_patterns_text, output_values_text = line.split('|')
    unique_patterns = unique_patterns_text.split()
    output_values = output_values_text.split()
    return unique_patterns, output_values

def get_lines(filename):
    with open(filename) as f:
        return f.readlines()

def product(iterable):
    result = 1
    for x in iterable:
        result *= x
    return result

if __name__ == '__main__':
    main()
