#!/usr/bin/env python3

INPUT_FILE = 'input09.txt'
# INPUT_FILE = 'example09.txt'

def main():
    lines = get_lines(INPUT_FILE)
    grid = parse_lines(lines)
    print(sum(get_risks(grid)))

def get_risks(grid):
    return [grid[r][c] + 1
            for r in range(len(grid))
            for c in range(len(grid[0]))
            if is_risk(grid, r, c)
            ]

def is_risk(grid, row_i, col_i):
    cell = grid[row_i][col_i]
    neighbors = get_neighbors(grid, row_i, col_i)
    return all(cell < neighbor for neighbor in neighbors)

ROW_COL_DIFFS = [
    (1, 0),
    (-1, 0),
    (0, 1),
    (0, -1),
]
def get_neighbors(grid, row_i, col_i):
    neighbor_indices = [(row_i + row_diff, col_i + col_diff)
                        for row_diff, col_diff in ROW_COL_DIFFS]
    row_range = range(0, len(grid))
    col_range = range(0, len(grid[0]))
    return [grid[r][c] for r, c in neighbor_indices
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

if __name__ == '__main__':
    main()
