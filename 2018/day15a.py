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

    def get_position(self):
        return self.row, self.col

    def tick(self, units):
        pass

    def get_targets(self, units):
        return {unit for unit in units if unit.team != self.team}

    def select_target(self, targets):
        pass

################################################################################
# Solution
################################################################################

def solve(units):
    num_rounds = run_combat(units)
    total_hp = sum(unit.hp for unit in units)
    return num_rounds * total_hp

def run_combat(units):
    num_rounds = 0
    game_done = False
    while not game_done:
        for unit in sorted(units, key=lambda unit: unit.get_position()):
            game_done = unit.tick()
            if game_done:
                break
        else:
            num_rounds += 1
    return num_rounds

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
    return units

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
    units = get_input()
    print(solve(units))

if __name__ == '__main__':
    main()
