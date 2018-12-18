#!/usr/bin/env python3
#pylint: disable=invalid-name

import sys
import re
from enum import Enum, auto

################################################################################
# Classes
################################################################################
class Square(Enum):
    SAND = auto()
    CLAY = auto()
    WATER = auto()
    # WATER_FALLING = auto()
    # WATER_AT_REST = auto()

    def to_char(self):
        if self == Square.SAND:
            return '.'
        if self == Square.CLAY:
            return '#'
        if self == Square.WATER:
            return '|'
        raise Exception('Invalid Square value:', self)

################################################################################
# Run
################################################################################
SPRING_X, SPRING_Y = 500, 0
SPRING_ROW, SPRING_COL = SPRING_Y, SPRING_X
def solve(grid):
    print(create_grid_string(grid))
    # return grid

def fill_with_water(grid, spring_location):
    spring_row, spring_col = spring_location
    min_row, max_row = min(grid), max(grid)
    row, col = min_row, spring_col
    while row <= max_row:
        pass

def create_grid_string(grid):
    lines = []
    for _, row in sorted(grid.items()):
        line = []
        for _, square in sorted(row.items()):
            line.append(square.to_char())
        lines.append(line)
    return '\n'.join(''.join(line) for line in lines)

################################################################################
# Input
################################################################################
def get_input():
    coordinates = set()
    for line in sys.stdin.readlines():
        coordinates.update(parse_line(line.strip()))
    return build_grid(coordinates)

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

def build_grid(coordinates):
    x_values = {x for x, _ in coordinates}
    y_values = {y for _, y in coordinates}
    min_x, max_x = min(x_values), max(x_values)
    min_y, max_y = min(y_values), max(y_values)
    grid = {}
    for y in range(min_y, max_y + 1):
        row = {}
        for x in range(min_x, max_x + 1):
            is_clay = (x, y) in coordinates
            row[x] = Square.CLAY if is_clay else Square.SAND
        grid[y] = row
    return grid

################################################################################
# Run
################################################################################
def main():
    grid = get_input()
    print(solve(grid))

if __name__ == '__main__':
    main()
