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
NUM_MINUTES = 1000000000
def solve(grid):
    board_to_index = {}
    index_to_board = {}
    for i in range(NUM_MINUTES):
        # if i % 10 == 0:
        #     print('{} ({:.2f}%)'.format(i, i / NUM_MINUTES))
        serialized_grid = serialize_grid(grid)
        answer = find_answer(board_to_index, index_to_board, serialized_grid, i)
        if answer is not None:
            return answer
        index_to_board[i] = serialize_grid(grid), calculate_resource_value(grid)
        board_to_index[serialized_grid] = i
        grid = next_grid(grid)
    return calculate_resource_value(grid)

def find_answer(board_to_index, index_to_board, serialized_grid, index):
    if serialized_grid not in board_to_index:
        return None
    start, end = board_to_index[serialized_grid], index
    length = end - start
    answer_cycle_index = (NUM_MINUTES - start) % length
    answer_absolute_index = start + answer_cycle_index
    _, resource_value = index_to_board[answer_absolute_index]
    return resource_value

SERIALIZED_SQUARE_LENGTH = 2
SLICE_SIZE = 8
SQUARES_PER_SLICE = SLICE_SIZE // SERIALIZED_SQUARE_LENGTH
def serialize_grid(grid):
    return bytes(serialize_square_slice(square_slice) for square_slice in
                 slices_of(flatten_gen(grid), SQUARES_PER_SLICE))

def serialize_square_slice(square_slice):
    num = 0
    for i, square in enumerate(square_slice):
        num += serialize_square(square) << (2*i)
    return num

def serialize_square(square):
    if square.is_open_ground():
        return 0
    if square.is_lumberyard():
        return 1
    return 2

def slices_of(enumerable, length):
    cur = []
    for value in enumerable:
        cur.append(value)
        if len(cur) == length:
            yield cur
            cur = []
    if cur:
        yield cur

def flatten_gen(grid):
    for row in grid:
        for square in row:
            yield square

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
