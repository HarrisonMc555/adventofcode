#!/usr/bin/env python3
#pylint: disable=invalid-name, too-few-public-methods

import sys
from enum import Enum, auto

################################################################################
# Classes
################################################################################

class Team(Enum):
    ELF = auto()
    GOBLIN = auto()

class Tile:
    def __init__(self, is_wall, occupant=None):
        self.is_wall = is_wall
        self.occupant = occupant

    def is_occupied(self):
        return self.is_wall or self.occupant is not None

    def has_unit(self):
        return self.occupant is not None

    def get_unit(self):
        if self.occupant is None:
            raise Exception('No unit at tile')
        return self.occupant

class Unit:
    STARTING_HP = 200

    def __init__(self, row, col, team, grid):
        self.row = row
        self.col = col
        self.team = team
        self.grid = grid
        self.hp = Unit.STARTING_HP

################################################################################
# Solution
################################################################################

def solve(grid):
    num_rounds, units = run_combat(grid)
    total_hp = sum(unit.hp for unit in units)
    return num_rounds * total_hp

def run_combat(grid):
    return 0, []

################################################################################
# Input
################################################################################

def get_input():
    char_grid = [line.strip() for line in sys.stdin.readlines()]
    return build_grid(char_grid)

def build_grid(char_grid):
    units = set()
    grid = []
    for i, char_row in enumerate(char_grid):
        row = []
        for j, char in enumerate(char_row):
            row.append(parse_char(char, i, j, grid, units))
        grid.append(row)
    return grid

def parse_char(char, row, col, grid, units):
    if char == '#':
        return Tile(True)
    if char == '.':
        return Tile(False)
    if char == 'E':
        elf = Unit(row, col, Team.ELF, grid)
        units.add(elf)
        return Tile(False, elf)
    if char == 'G':
        goblin = Unit(row, col, Team.GOBLIN, grid)
        units.add(goblin)
        return Tile(False, goblin)
    raise Exception('Invalid tile char')

################################################################################
# Run
################################################################################

def main():
    pattern = get_input()
    print(solve(pattern))

if __name__ == '__main__':
    main()
