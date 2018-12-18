#!/usr/bin/env python3
#pylint: disable=invalid-name, too-few-public-methods

import sys
from enum import Enum, auto

################################################################################
# Classes
################################################################################
class Square(Enum):
    OPEN_GROUND = auto()
    TREES = auto()
    LUMBERYARD = auto()

    def tick(self, row, col, grid):
        surrounding_squares = get_surrounding_squares(row, col, grid)
        if self.is_open_ground():
            num_wooded = count(surrounding_squares, Square.is_trees)
            return Square.TREES if num_wooded >= 3 else self
        if self.is_trees():
            num_lumberyards = count(surrounding_squares, Square.is_lumberyard)
            return Square.LUMBERYARD if num_lumberyards >= 3 else self
        if self.is_lumberyard():
            num_wooded = count(surrounding_squares, Square.is_trees)
            num_lumberyards = count(surrounding_squares, Square.is_lumberyard)
            return self if num_lumberyards >= 1 and num_wooded >= 1 else \
                Square.OPEN_GROUND
        raise Exception('Invalid square type', self)

    def to_char(self):
        return \
            '.' if self == Square.OPEN_GROUND else \
            '|' if self == Square.TREES else \
            '#'

    def is_open_ground(self):
        return self == Square.OPEN_GROUND

    def is_trees(self):
        return self == Square.TREES

    def is_lumberyard(self):
        return self == Square.LUMBERYARD

    @staticmethod
    def from_char(char):
        square = \
            Square.OPEN_GROUND if char == '.' else \
            Square.TREES if char == '|' else \
            Square.LUMBERYARD if char == '#' else \
            None
        if not square:
            raise Exception('Invalid square char', char)
        return square

def get_surrounding_squares(row, col, grid):
    num_rows, num_cols = len(grid), len(grid[0])
    indices = get_surrounding_indices(row, col, num_rows, num_cols)
    return [grid[i][j] for i, j in indices]

def get_surrounding_indices(row, col, num_rows, num_cols):
    min_row, max_row = max(0, row - 1), min(num_rows - 1, row + 1)
    min_col, max_col = max(0, col - 1), min(num_cols - 1, col + 1)
    return [(i, j)
            for j in range(min_col, max_col + 1)
            for i in range(min_row, max_row + 1)
            # don't include yourself
            if (i, j) != (row, col)]

################################################################################
# Solve
################################################################################
NUM_MINUTES = 10
def solve(grid):
    # print(create_grid_string(grid))
    for _ in range(NUM_MINUTES):
        grid = next_grid(grid)
        # print()
        # print(create_grid_string(grid))
    return calculate_resource_value(grid)

def next_grid(grid):
    return [[square.tick(i, j, grid) for j, square in enumerate(row)]
            for i, row in enumerate(grid)]

def calculate_resource_value(grid):
    num_wooded = count2d(grid, Square.is_trees)
    num_lumberyards = count2d(grid, Square.is_lumberyard)
    return num_wooded * num_lumberyards

def count2d(grid, fun):
    return sum(count(row, fun) for row in grid)

def count(enumerable, fun):
    return sum(1 for value in enumerable if fun(value))

def create_grid_string(grid):
    return '\n'.join(''.join(square.to_char() for square in row)
                     for row in grid)

################################################################################
# Input
################################################################################
def get_input():
    return [parse_line(line.strip()) for line in sys.stdin.readlines()]

def parse_line(line):
    return [Square.from_char(char) for char in line]

################################################################################
# Run
################################################################################
def main():
    grid = get_input()
    print(solve(grid))

if __name__ == '__main__':
    main()
