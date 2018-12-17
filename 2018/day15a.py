#!/usr/bin/env python3
#pylint: disable=invalid-name, too-few-public-methods

import sys
from enum import Enum, auto

################################################################################
# Classes
################################################################################

class NoMoreTargetsException(Exception):
    pass

class NoReachablePositions(Exception):
    pass

class Team(Enum):
    ELF = auto()
    GOBLIN = auto()

class Tile:
    def __init__(self, is_wall, occupant=None):
        self.is_wall = is_wall
        self.occupant = occupant

    def is_occupied(self):
        return self.is_wall or self.occupant is not None

    def is_unoccupied(self):
        return not self.is_occupied()

    def has_unit(self):
        return self.occupant is not None

    def get_unit(self):
        if self.occupant is None:
            raise Exception('No unit at tile')
        return self.occupant

    def remove_occupant(self):
        if self.occupant is None:
            raise Exception('Remove occupant from empty tile')
        self.occupant = None

    def add_occupant(self, occupant):
        if self.occupant is not None:
            raise Exception('Add occupant to occupied tile')
        self.occupant = occupant

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
        did_move = self.move(units)
        killed_unit = self.attack(units)
        return did_move, killed_unit

    def move(self, units):
        targets = self.get_targets(units)
        if not targets:
            raise NoMoreTargetsException()
        target_adjacent_positions = get_unit_valid_adjacent_positions(targets)
        if self.get_position() in target_adjacent_positions:
            return False
        destination = self.select_destination(target_adjacent_positions)
        step_position = self.select_first_step_towards(destination)
        if step_position and step_position != self.get_position():
            self.move_to(step_position)
            return True
        return False

    def get_targets(self, units):
        return {unit for unit in units if unit.team != self.team}

    def select_destination(self, destinations):
        closest_positions = select_closest_positions(self.get_position(),
                                                     destinations, self.grid)
        assert closest_positions
        # choose based on reading order
        return sorted(closest_positions)[0]

    def select_first_step_towards(self, position):
        closest_to_me = select_closest_positions(position, self.get_position(),
                                                 self.grid)
        assert closest_to_me
        # choose based on reading order
        return sorted(closest_to_me)[0]

    def move_to(self, destination):
        self.grid[self.row][self.col].remove_occupant()
        row, col = destination
        self.row = row
        self.col = col
        self.grid[self.row][self.col].add_occupant(self)

    def get_valid_adjacent_positions(self):
        return get_valid_adjacent_positions(self.get_position(), self.grid)

    def attack(self, units):
        if units:
            raise Exception('Unimplemented')
        return self

def get_unit_valid_adjacent_positions(units):
    positions = set()
    for unit in units:
        positions.update(unit.get_valid_adjacent_positions())
    return positions

DIDJS = [(1, 0), (-1, 0), (0, 1), (0, -1)]
def get_adjacent_positions(position):
    i, j = position
    return [(i + di, j + dj) for di, dj in DIDJS]

def get_valid_adjacent_positions(position, grid):
    return [(i, j) for i, j in get_adjacent_positions(position)
            if grid[i][j].is_unoccupied()]

def select_closest_positions(start, destinations, grid):
    cur_positions = {start}
    visited = set()
    closest_positions = set()
    while not closest_positions:
        next_positions = set()
        if not cur_positions:
            raise NoReachablePositions()
        for position in cur_positions:
            if position in destinations:
                closest_positions.add(position)
            else:
                next_positions.update(get_valid_adjacent_positions(position,
                                                                   grid))
            visited.add(position)
        next_positions.difference_update(visited)
        cur_positions = next_positions
    return closest_positions

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
            try:
                unit.tick()
            except NoMoreTargetsException:
                game_done = True
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
