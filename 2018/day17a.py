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
    # WATER = auto()
    WATER_FALLING = auto()
    WATER_AT_REST = auto()

    def is_sand(self):
        return self == Square.SAND

    def is_clay(self):
        return self == Square.CLAY

    def is_water_falling(self):
        return self == Square.WATER_FALLING

    def is_water_at_rest(self):
        return self == Square.WATER_AT_REST

    def is_water(self):
        return self.is_water_falling() or self.is_water_at_rest()

    def is_supportive(self):
        return self.is_clay() or self.is_water_at_rest()

    def to_char(self):
        if self == Square.SAND:
            return '.'
        if self == Square.CLAY:
            return '#'
        # if self == Square.WATER:
        #     return '|'
        if self == Square.WATER_FALLING:
            return '|'
        if self == Square.WATER_AT_REST:
            return '~'
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
    # print(grid.create_printable_string())
    fill_with_water(grid, SPRING_COL)
    # print()
    # print(grid.create_printable_string())
    return count_water_squares(grid)

def count_water_squares(grid):
    num_water_squares = 0
    for row in grid.squares.values():
        for square in row.values():
            if square.is_water():
                num_water_squares += 1
    return num_water_squares

def fill_with_water(grid, spring_col):
    location = grid.min_row, spring_col
    stack = [(fill_down_from, location)]
    while stack:
        function, location = stack.pop()
        new_entries = function(grid, location)
        stack.extend(new_entries)

def fill_down_from(grid, location):
    row, col = location
    if grid[row][col].is_water():
        return []
    while row <= grid.max_row and grid[row][col].is_sand():
        grid[row][col] = Square.WATER_FALLING
        row += 1
    if row <= grid.max_row and grid[row][col].is_supportive():
        return [(fill_across_from, (row - 1, col))]
    return []

def fill_across_from(grid, location):
    new_entries = []
    row, start_col = location
    # go left
    col = start_col
    # square must be sand or water (i.e. not clay)
    # below must be supportive
    while not grid[row][col].is_clay() and grid[row + 1][col].is_supportive():
        grid[row][col] = Square.WATER_FALLING
        col -= 1
    if grid[row][col].is_sand() and grid[row + 1][col].is_sand():
        new_entries.append((fill_down_from, (row, col)))
    hit_wall_left = grid[row][col].is_clay()
    left_col = col + 1
    # go right
    col = start_col + 1
    while not grid[row][col].is_clay() and grid[row + 1][col].is_supportive():
        grid[row][col] = Square.WATER_FALLING
        col += 1
    if grid[row][col].is_sand() and grid[row + 1][col].is_sand():
        new_entries.append((fill_down_from, (row, col)))
    hit_wall_right = grid[row][col].is_clay()
    right_col = col - 1
    # possibly go up
    if hit_wall_left and hit_wall_right:
        new_entries.append((fill_across_from, (row - 1, start_col)))
        for col in range(left_col, right_col + 1):
            grid[row][col] = Square.WATER_AT_REST
    return new_entries

################################################################################
# Input
################################################################################
def get_input():
    coordinates = set()
    for line in sys.stdin.readlines():
        coordinates.update(parse_line(line.strip()))
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

################################################################################
# Run
################################################################################
def main():
    grid = get_input()
    print(solve(grid))

if __name__ == '__main__':
    main()
