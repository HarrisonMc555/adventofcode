#!/usr/bin/env python3
#pylint: disable=invalid-name, too-few-public-methods

import sys
from enum import Enum, auto

DEBUG_COUNTER = 0

def dprint(*args, **kwargs):
    if DEBUG_COUNTER > 0:
        print(*args, **kwargs)

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

    def to_char(self):
        if self == Team.ELF:
            return 'E'
        if self == Team.GOBLIN:
            return 'G'
        raise Exception('Invalid team')

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

    def to_char(self):
        if self.occupant:
            return self.occupant.team.to_char()
        if self.is_wall:
            return '#'
        return '.'

class Unit:
    STARTING_HP = 200
    DEFAULT_ATTACK_POWER = 3

    def __init__(self, row, col, team, grid):
        self.row = row
        self.col = col
        self.team = team
        self.grid = grid
        self.hp = Unit.STARTING_HP
        self.attack_power = Unit.DEFAULT_ATTACK_POWER
        self.last_target = None

    def __str__(self):
        return '<({}, {}), {}, {}/{}>'.format(self.row, self.col, self.team,
                                              self.hp, Unit.STARTING_HP)

    def get_position(self):
        return self.row, self.col

    def tick(self, units, nothing_changed_last_time):
        if not nothing_changed_last_time:
            did_move = self.move(units)
        else:
            did_move = False
        killed_unit = self.attack(units, nothing_changed_last_time)
        return did_move, killed_unit

    def move(self, units):
        dprint('\n\n\n')
        dprint('moving', self)
        dprint('\t', 'calling get_targets')
        targets = self.get_targets(units)
        dprint('\t', 'targets:', ', '.join(str(t) for t in targets))
        if not targets:
            dprint('\t', '!!! no targets found !!!')
            raise NoMoreTargetsException()
        dprint('\t', 'calling get_unit_valid_adjacent_positions')
        if self.get_adjacent_targets(units):
            return False
        target_adjacent_positions = get_unit_valid_adjacent_positions(targets)
        dprint('\t', 'target_adjacent_positions', target_adjacent_positions)
        if self.get_position() in target_adjacent_positions:
            dprint('\t', '!!! already adjacent to a target !!!')
            return False
        try:
            dprint('\t', 'calling select_destination')
            destination = self.select_destination(target_adjacent_positions)
        except NoReachablePositions:
            return False
        dprint('\t', 'calling select_first_step_towards')
        step_position = self.select_first_step_towards(destination)
        dprint('\t', 'step_position:', step_position)
        if step_position != self.get_position():
            dprint('\t', 'calling move_to')
            self.move_to(step_position)
            return True
        dprint('\t', 'Already at', step_position)
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
        adjacent_positions = get_valid_adjacent_positions(self.get_position(),
                                                          self.grid)
        closest_to_me = select_closest_positions(position, adjacent_positions,
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

    def is_adjacent_to(self, unit):
        return is_adjacent_to(self.get_position(), unit.get_position())

    def attack(self, units, nothing_changed_last_time):
        if nothing_changed_last_time:
            target = self.last_target
        else:
            dprint('\n')
            dprint('attack:', self)
            targets = self.get_targets(units)
            if not targets:
                raise NoMoreTargetsException()
            target = self.select_adjacent_target(units)
        if not target:
            self.last_target = None
            return None
        is_dead = target.attacked(self.attack_power)
        self.last_target = target if not is_dead else None
        return target if is_dead else None

    def get_adjacent_targets(self, units):
        targets = self.get_targets(units)
        dprint('\t', 'targets:', ', '.join(str(t) for t in targets))
        return {target for target in targets if self.is_adjacent_to(target)}

    def select_adjacent_target(self, units):
        adjacent_targets = self.get_adjacent_targets(units)
        if not adjacent_targets:
            return None
        assert adjacent_targets
        # choose based on lowest hp, then reading order
        return sorted(adjacent_targets,
                      key=lambda t: (t.hp, t.get_position()))[0]

    def attacked(self, attack_power):
        self.hp -= attack_power
        if self.hp <= 0:
            self.grid[self.row][self.col].remove_occupant()
        return self.hp <= 0

def get_unit_valid_adjacent_positions(units):
    positions = set()
    for unit in units:
        positions.update(unit.get_valid_adjacent_positions())
    return positions

DIDJS = [(1, 0), (-1, 0), (0, 1), (0, -1)]
def get_adjacent_positions(position):
    i, j = position
    return [(i + di, j + dj) for di, dj in DIDJS]

def is_adjacent_to(position1, position2):
    i, j = position1
    return any(position2 == (i + di, j + dj) for di, dj in DIDJS)

def get_valid_adjacent_positions(position, grid):
    return [(i, j) for i, j in get_adjacent_positions(position)
            if grid[i][j].is_unoccupied()]

def select_closest_positions(start, destinations, grid):
    dprint('select_closest_positions')
    dprint('\t', start, '->', ', '.join(str(d) for d in destinations))
    cur_positions = {start}
    visited = set()
    closest_positions = set()
    while not closest_positions:
        dprint('\t\t', 'cur_positions:', cur_positions)
        next_positions = set()
        if not cur_positions:
            raise NoReachablePositions()
        for position in cur_positions:
            dprint('\t\t\t', 'position:', position)
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

def test(units):
    before_grid_string = create_grid_string(next(iter(units)).grid, units)
    num_rounds = run_combat(units)
    total_hp = sum(unit.hp for unit in units)
    after_grid_string = create_grid_string(next(iter(units)).grid, units,
                                           show_hp=True)
    lines = []
    mid_index = len(lines) // 2
    for i, (line1, line2) in enumerate(zip(before_grid_string.split('\n'),
                                           after_grid_string.split('\n'))):
        mid = '  -->  ' if i == mid_index else ' '*7
        lines.append(line1 + mid + line2)
    print('\n'.join(lines))
    print()
    print('Combat ends after {} full rounds'.format(num_rounds))
    winners = next(iter(units)).team
    winners_string = 'Goblins' if winners == Team.GOBLIN else 'Elves'
    print('{} win with {} total hit points left'.format(winners_string,
                                                        total_hp))
    print('Outcome: {} * {} = {}'.format(num_rounds, total_hp,
                                         num_rounds * total_hp))

def run_combat(units):
    num_rounds = 0
    game_done = False
    nothing_changed_last_time = False
    while not game_done:
        something_changed = False
        killed_units = set()
        for unit in sorted(units, key=lambda unit: unit.get_position()):
            if unit in killed_units:
                continue
            try:
                this_unit_moved, unit_killed = unit.tick(
                    units, nothing_changed_last_time and not something_changed)
                something_changed |= this_unit_moved or bool(unit_killed)
                if unit_killed:
                    units.remove(unit_killed)
                    killed_units.add(unit_killed)
            except NoMoreTargetsException:
                # print('NoMoreTargetsException')
                game_done = True
                break
        else:
            num_rounds += 1
        nothing_changed_last_time = not something_changed
    return num_rounds

def create_grid_string(grid, units, show_hp=False):
    lines = []
    for i, row in enumerate(grid):
        line = ''.join(tile.to_char() for tile in row)
        if show_hp:
            units_on_line = {unit for unit in units if unit.get_position()[0] == i}
            sorted_units = sorted(units_on_line, key=lambda u: u.get_position())
            if sorted_units:
                line += '   ' + ', '.join(create_hp_string(u) for u in sorted_units)
        lines.append(line)
    return '\n'.join(lines)

def create_hp_string(unit):
    return '{}({})'.format(unit.team.to_char(), unit.hp)

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
