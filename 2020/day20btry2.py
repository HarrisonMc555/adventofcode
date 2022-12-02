#!/usr/bin/env python3

import re
import math
import unittest
from dataclasses import dataclass
from collections import defaultdict
from enum import Enum, auto

# INPUT_FILE = 'input20.txt'
INPUT_FILE = 'example20.txt'

TILE_RE = re.compile('Tile (\d+):')

def main():
    if True:
        unittest.main()
        return
    text = get_text(INPUT_FILE)
    tiles = parse_tiles(text)
    # corners = [tile.tile_id for tile in tiles
    #            if num_neighbors(tiles, tile) == 2]
    # top_left = corners[0]
    # top_left = find_corner(tiles)
    # tile_id_grid = assemble_tile_id_grid(tiles, top_left)
    # print(corners)
    # print(product(corners))
    for tile in tiles[:2]:
        print(tile.tile_id)
        print('\n'.join(tile.tile_lines))
        print()
        print('mods:')
        # for mod, code in tile.mod_to_code.items():
        #     print(f'{mod} -> {code}')
        for side, code in tile.side_to_code.items():
            print(f'{side} -> {code}')
        print()
        print()
    for tile in tiles:
        print(f'{tile.tile_id} has {num_neighbors(tiles, tile)} neighbors')
    print()
    print()

    # top_left = find_corner(tiles)
    # print('Top left:')
    # print(top_left.tile_id)
    # print('\n'.join(top_left.tile_lines))
    # print()
    # print('mods:')
    # for mod, code in top_left.mod_to_code.items():
    #     print(f'{mod} -> {code}')
    # print()
    # print()
    grid = assemble_tile_id_grid(tiles)

def assemble_tile_id_grid(tiles):
    tile_id_to_tile = {tile.tile_id: tile for tile in tiles}
    width = int(math.sqrt(len(tiles)))
    code_to_tiles = create_code_to_tiles(tiles)
    top_left = find_corner(tiles)

    # Find top left orientation.
    for rotation in range(4):
        modification = Modification(rotation, False)
        right_code = top_left.code(modification, Direction.RIGHT)
        bottom_code = top_left.code(modification, Direction.BOTTOM)
        if len(code_to_tiles[right_code]) == 2 and \
           len(code_to_tiles[bottom_code]) == 2:
            print(f'right: {right_code:010b} ({right_code}), bottom: {bottom_code:010b} ({bottom_code}), rotation: {rotation}')
            print()
            break
    else:
        raise Exception('No modification found for top left')
    top_row = [(top_left, modification)]

    # Fill top row.
    for column_index in range(1, width):
        prev_tile, prev_mod = top_row[-1]
        top_row.append(get_tile_next_to(code_to_tiles, prev_tile, prev_mod, Direction.RIGHT))
        test_print_grid([top_row])

    # Fill later rows.
    grid = [top_row]
    for row_index in range(1, width):
        # Find left-most tile.
        prev_tile, prev_mod = grid[-1][0]
        first_entry = get_tile_next_to(code_to_tiles, prev_tile, prev_mod, Direction.BOTTOM)
        new_row = [first_entry]
        test_print_grid(grid + new_row)
        for column_index in range(1, width):
            prev_tile, prev_mod = new_row[-1]
            new_row.append(get_tile_next_to(code_to_tiles, prev_tile, prev_mod,
                                            Direction.RIGHT))
            test_print_grid(grid + new_row)
        grid.append(new_row)
        test_print_grid(grid)

    for row_index in range(len(top_left.tile_lines)):
        for tile, _ in top_row:
            print(tile.tile_lines[row_index], end='')
            print(' ', end='')
        print()
    print()
    print(top_row)


def get_tile_next_to(code_to_tiles, prev_tile, prev_mod, direction):
    code = prev_tile.code(prev_mod, direction)
    print(f'prev_tile: {prev_tile.tile_id}')
    print(f'prev_mod: {prev_mod}')
    print(f'code: {code}')
    next_tiles = [tile for tile in code_to_tiles[code]
                 if tile.tile_id != prev_tile.tile_id]
    if len(next_tiles) != 1:
        print(f'next_tiles: {next_tiles}')
        raise Exception('Expected exactly one match')
    next_tile = next_tiles[0]
    print(f'next_tile: {next_tile.tile_id}')
    for side, next_code in next_tile.side_to_code.items():
        if next_code != code:
            continue
        print(f'Adding {side} -> {side.modification(direction.opposite())}')
        print()
        return (next_tile, side.modification(direction.opposite()))
    else:
        raise Exception(f'Did not find matching code {code}')

def create_code_to_tiles(tiles):
    code_to_tiles = defaultdict(list)
    for tile in tiles:
        for code in tile.codes():
            code_to_tiles[code].append(tile)
    return code_to_tiles

def find_corner(tiles):
    for tile in tiles:
        if num_neighbors(tiles, tile) == 2:
            return tile
    raise Exception('No corner found')

def num_neighbors(tiles, tile):
    total = 0
    for other_tile in tiles:
        if other_tile.tile_id == tile.tile_id:
            continue
        if tile.matches(other_tile):
            total += 1
    return total

def parse_tiles(text):
    return [parse_tile(tile) for tile in text.split('\n\n')]

def parse_tile(text):
    lines = text.split('\n')
    id_line = lines[0]
    tile_id = int(TILE_RE.match(id_line).groups()[0])
    tile_lines = lines[1:]
    tile = Tile.create(tile_id, tile_lines)
    return tile

@dataclass(frozen=True)
class Modification:
    rotation: int
    flipped: bool

    @staticmethod
    def all():
        return [Modification(rotation, flipped)
                for rotation in range(4)
                for flipped in [False, True]]

class Direction(Enum):
    TOP = auto()
    RIGHT = auto()
    BOTTOM = auto()
    LEFT = auto()

    def opposite(self):
        return add_wrapping_to_enum(self, 2)

# TOP = 0
# RIGHT = 1
# BOTTOM = 2
# LEFT = 3

@dataclass(frozen=True)
class Side:
    direction: Direction
    clockwise: bool

    @staticmethod
    def modified(modification, direction):
        # TOP = 0
        # RIGHT = 1
        # BOTTOM = 2
        # LEFT = 3

        # 0 NF top = top
        # 0 NF right = right
        # 0 NF bottom = bottom
        # 0 NF left = left

        # 1 NF top = left
        # 1 NF right = top
        # 1 NF bottom = right
        # 1 NF left = bottom

        # 2 NF top = bottom
        # 2 NF right = left
        # 2 NF bottom = top
        # 2 NF left = right

        # 3 NF top = left
        # 3 NF right = top
        # 3 NF bottom = right
        # 3 NF left = bottom

        multiplier = 3 if modification.flipped else -1
        # multiplier = -1
        add = modification.rotation * multiplier
        # new_value = (direction.value - 1) * multiplier + add
        original_value = direction.value - 1
        starting_value = original_value * -1 if modification.flipped else \
            original_value
        # new_value = (direction.value - 1) + add
        new_value = starting_value + add
        new_direction = Direction(new_value % 4 + 1)
        # direction = add_wrapping_to_enum(direction, add)
        # return Side(direction, not modification.flipped)
        return Side(new_direction, not modification.flipped)

    def modification(self, direction):
        multiplier = 1 if self.clockwise else -1
        add = self.direction.value * multiplier
        direction = add_wrapping_to_enum(direction, add)
        return Modification(direction.value - 1, not self.clockwise)

@dataclass(frozen=True)
class Tile:
    tile_id: str
    tile_lines: [str]
    side_to_code: {Side: int}
    # mod_to_code: {Modification: int}
    # code_to_mod: {int: Modification}

    @staticmethod
    def create(tile_id, tile_lines):
        top = tile_lines[0]
        bottom = tile_lines[-1]
        left = ''.join(line[0] for line in tile_lines)
        right = ''.join(line[-1] for line in tile_lines)

        top_cw = line_to_code(top)
        top_ccw = line_to_code(reversed(top))
        right_cw = line_to_code(right)
        right_ccw = line_to_code(reversed(right))
        bottom_cw = line_to_code(reversed(bottom))
        bottom_ccw = line_to_code(bottom)
        left_cw = line_to_code(reversed(left))
        left_ccw = line_to_code(left)

        side_to_code = {
            Side(Direction.TOP, True): top_cw,
            Side(Direction.TOP, False): top_ccw,
            Side(Direction.RIGHT, True): right_cw,
            Side(Direction.RIGHT, False): right_ccw,
            Side(Direction.BOTTOM, True): bottom_cw,
            Side(Direction.BOTTOM, False): bottom_ccw,
            Side(Direction.LEFT, True): left_cw,
            Side(Direction.LEFT, False): left_ccw,
        }
        # mod_to_code = {
        #     Modification(TOP, False): top_cw,
        #     Modification(RIGHT, False): right_cw,
        #     Modification(BOTTOM, False): bottom_cw,
        #     Modification(LEFT, False): left_cw,
        #     Modification(TOP, True): top_ccw,
        #     Modification(RIGHT, True): left_ccw,
        #     Modification(BOTTOM, True): right_ccw,
        #     Modification(LEFT, True): bottom_ccw,
        # }
        # code_to_mod = {code: mod for mod, code in mod_to_code.items()}
        # return Tile(tile_id, tile_lines, mod_to_code, code_to_mod)
        return Tile(tile_id, tile_lines, side_to_code)

    def codes(self):
        # return self.code_to_mod.keys()
        return self.side_to_code.values()

    def matches(self, other_tile):
        return bool(set(self.codes()).intersection(other_tile.codes()))

    def code(self, modification, direction):
        return self.side_to_code[Side.modified(modification, direction)]

def line_to_code(line):
    result = 0
    for c in line:
        result *= 2
        if c == '#':
            result += 1
    return result

def get_text(filename):
    with open(filename) as f:
        return f.read().strip()

def parse_tiles(text):
    return [parse_tile(tile) for tile in text.split('\n\n')]

def parse_tile(text):
    lines = text.split('\n')
    id_line = lines[0]
    tile_id = int(TILE_RE.match(id_line).groups()[0])
    tile_lines = lines[1:]
    tile = Tile.create(tile_id, tile_lines)
    return tile

def add_wrapping_to_enum(variant, add):
    cls = type(variant)
    zero_based_value = variant.value - 1
    zero_based_next_value = (zero_based_value + add) % len(cls)
    next_value = zero_based_next_value + 1
    return cls(next_value)

def test_print_grid(grid):
    for row in grid:
        for tile, _ in row:
            print(f'{tile.tile_id} ', end='')
        print()
    print()

def test():
    for direction in Direction:
        assert Side.modified(Modification(0, False), direction) == \
            Side(direction, True)
    top = Direction.TOP
    right = Direction.RIGHT
    bottom = Direction.BOTTOM
    left = Direction.LEFT

    for in_dir, out_dir in [
            (top, left),
            (right, top),
            (bottom, right),
            (left, bottom),
    ]:
        assert Side.modified(Modification(1, False), in_dir) == \
            Side(out_dir, True)

    for in_dir, out_dir in [
            (top, bottom),
            (right, left),
            (bottom, top),
            (left, right),
    ]:
        assert Side.modified(Modification(2, False), in_dir) == \
            Side(out_dir, True)

    for in_dir, out_dir in [
            (top, right),
            (right, bottom),
            (bottom, left),
            (left, top),
    ]:
        assert Side.modified(Modification(3, False), in_dir) == \
            Side(out_dir, True)

    for in_dir, out_dir in [
            (top, top),
            (right, left),
            (bottom, bottom),
            (left, right),
    ]:
        # print(f'in_dir: {in_dir}, out_dir: {out_dir} -> {Side.modified(Modification(0, True), in_dir)}')
        assert Side.modified(Modification(0, True), in_dir) == \
            Side(out_dir, False)

class TestModifyMethods(unittest.TestCase):
    def test_side_modified(self):
        for direction in Direction:
            self.assertEqual(Side.modified(Modification(0, False), direction),
                             Side(direction, True))

        top = Direction.TOP
        right = Direction.RIGHT
        bottom = Direction.BOTTOM
        left = Direction.LEFT

        for in_dir, out_dir in [
                (top, left),
                (right, top),
                (bottom, right),
                (left, bottom),
        ]:
            self.assertEqual(Side.modified(Modification(1, False), in_dir),
                Side(out_dir, True))

        for in_dir, out_dir in [
                (top, bottom),
                (right, left),
                (bottom, top),
                (left, right),
        ]:
            self.assertEqual(Side.modified(Modification(2, False), in_dir),
                             Side(out_dir, True))

        for in_dir, out_dir in [
                (top, right),
                (right, bottom),
                (bottom, left),
                (left, top),
        ]:
            self.assertEqual(Side.modified(Modification(3, False), in_dir),
                             Side(out_dir, True))

        for in_dir, out_dir in [
                (top, top),
                (right, left),
                (bottom, bottom),
                (left, right),
        ]:
            # print(f'in_dir: {in_dir}, out_dir: {out_dir} -> {Side.modified(Modification(0, True), in_dir)}')
            self.assertEqual(Side.modified(Modification(0, True), in_dir),
                             Side(out_dir, False))

        for in_dir, out_dir in [
                (top, left),
                (right, top),
                (bottom, right),
                (left, bottom),
        ]:
            # print(f'in_dir: {in_dir}, out_dir: {out_dir} -> {Side.modified(Modification(0, True), in_dir)}')
            self.assertEqual(Side.modified(Modification(1, True), in_dir),
                             Side(out_dir, False))

if __name__ == '__main__':
    main()
