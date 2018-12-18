#!/usr/bin/env python3
#pylint: disable=invalid-name, too-few-public-methods

import sys
import re
from enum import Enum, auto
from collections import defaultdict

################################################################################
# Classes
################################################################################
class Square(Enum):
    SAND = auto()
    CLAY = auto()
    WATER = auto()
    # WATER_FALLING = auto()
    # WATER_AT_REST = auto()

    def is_sand(self):
        return self == Square.SAND

    def is_clay(self):
        return self == Square.CLAY

    def is_water(self):
        return self == Square.WATER

    def to_char(self):
        if self == Square.SAND:
            return '.'
        if self == Square.CLAY:
            return '#'
        if self == Square.WATER:
            return '|'
        raise Exception('Invalid Square value:', self)

class Grid:
    def __init__(self, coordinates):
        x_values = {x for x, _ in coordinates}
        y_values = {y for _, y in coordinates}
        min_x, max_x = min(x_values), max(x_values)
        min_y, max_y = min(y_values), max(y_values)
        # you can overflow from the sides of the edges, but that's as far in x
        # as you'll ever go
        min_x -= 1
        max_x += 1
        self.squares = defaultdict(lambda: defaultdict(lambda: Square.SAND))
        for x, y in coordinates:
            self.squares[y][x] = Square.CLAY
        self.min_row, self.max_row = min_y, max_y
        self.min_col, self.max_col = min_x, max_x

    def __getitem__(self, indices):
        return self.squares[indices]

    def create_printable_string(self):
        lines = []
        for row in range(self.min_row, self.max_row + 1):
            line = []
            for col in range(self.min_col, self.max_col + 1):
                line.append(self.squares[row][col].to_char())
            lines.append(line)
        return '\n'.join(''.join(line) for line in lines)

################################################################################
# Run
################################################################################
SPRING_COL = 500
def solve(grid):
    print(grid.create_printable_string())
    fill_with_water(grid, SPRING_COL)
    print(grid.create_printable_string())
    return count_water_squares(grid)

def count_water_squares(grid):
    num_water_squares = 0
    for row in grid.squares.values():
        for square in row.values():
            if square.is_water():
                num_water_squares += 1
    return num_water_squares

def fill_with_water(grid, spring_col):
    row, col = grid.min_row, spring_col
    fill_down_from(grid, (row, col))

def fill_down_from(grid, location):
    print('fill_down_from:', location)
    row, col = location
    if grid[row][col].is_water():
        print('\t', 'exiting early')
        return
    while row <= grid.max_row and not grid[row][col].is_clay():
        # print('\t', 'water fell down to', (row, col))
        grid[row][col] = Square.WATER
        row += 1
    # print(grid.create_printable_string())
    if row <= grid.max_row and grid[row][col].is_clay():
        fill_across_from(grid, (row - 1, col))

def fill_across_from(grid, location):
    print('fill_across_from:', location)
    row, start_col = location
    # go left
    col = start_col
    # square must be sand or water (i.e. not clay)
    # below must be clay or water (i.e. not sand)
    # print('\tgoing left...')
    # while not grid[row][col].is_clay() and not grid[row + 1][col].is_sand():
    while not grid[row][col].is_clay() and not grid[row + 1][col].is_sand():
        # print('\t', 'water spread across to', (row, col))
        grid[row][col] = Square.WATER
        col -= 1
    if grid[row][col].is_sand() and grid[row + 1][col].is_sand():
        # print('\t', 'falling over to the left at', (row, col))
        fill_down_from(grid, (row, col))
    hit_wall_left = grid[row][col].is_clay()
    # go right
    col = start_col + 1
    # print('\tgoing right...')
    while not grid[row][col].is_clay() and not grid[row + 1][col].is_sand():
        # print('\t', 'water spread across to', (row, col))
        grid[row][col] = Square.WATER
        col += 1
    if grid[row][col].is_sand() and grid[row + 1][col].is_sand():
        # print('\t', 'falling over to the right at', (row, col))
        fill_down_from(grid, (row, col))
    hit_wall_right = grid[row][col].is_clay()
    # possibly go up
    if hit_wall_left and hit_wall_right:
        # print('\tgoing up...')
        fill_across_from(grid, (row - 1, start_col))

################################################################################
# Input
################################################################################
def get_input():
    coordinates = set()
    for line in sys.stdin.readlines():
        coordinates.update(parse_line(line.strip()))
    # return build_grid(coordinates)
    return Grid(coordinates)

PATTERN = re.compile(r'([xy])=(\d+), ([xy])=(\d+)..(\d+)')
def parse_line(line):
    xy1, val1, xy2, val2min, val2max = PATTERN.match(line).groups()
    val1, val2min, val2max = int(val1), int(val2min), int(val2max)
    coordinates = [(val1, val2) for val2 in range(val2min, val2max + 1)]
    assert xy1 != xy2
    assert sorted([xy1, xy2]) == ['x', 'y']
    if xy1 == 'y':
        coordinates = [(b, a) for (a, b) in coordinates]
    return coordinates

# def build_grid(coordinates):
#     x_values = {x for x, _ in coordinates}
#     y_values = {y for _, y in coordinates}
#     min_x, max_x = min(x_values), max(x_values)
#     min_y, max_y = min(y_values), max(y_values)
#     grid = {}
#     for x, y in coordinates:
#         grid[y][x] = Square.CLAY
#     for y in range(min_y, max_y + 1):
#         row = defaultdict(lambda: Square.SAND)
#         for x in range(min_x, max_x + 1):
#             is_clay = (x, y) in coordinates
#             row[x] = Square.CLAY if is_clay else Square.SAND
#         grid[y] = row
#     return grid

################################################################################
# Run
################################################################################
def main():
    grid = get_input()
    print(solve(grid))

if __name__ == '__main__':
    main()
