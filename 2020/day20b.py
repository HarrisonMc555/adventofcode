#!/usr/bin/env python3

import re
import math
from dataclasses import dataclass
from collections import defaultdict
from enum import Enum, auto

# INPUT_FILE = 'input20.txt'
INPUT_FILE = 'example20.txt'

TILE_RE = re.compile('Tile (\d+):')

import pudb; pu.db

def main():
    text = get_text(INPUT_FILE)
    tiles = parse_tiles(text)
    # corners = [tile.tile_id for tile in tiles
    #            if num_neighbors(tiles, tile) == 2]
    # top_left = corners[0]
    top_left = find_corner(tiles)
    tile_id_grid = assemble_tile_id_grid(tiles, top_left)
    # print(corners)
    # print(product(corners))

def assemble_tile_id_grid(tiles, top_left):
    tile_id_to_tile = {tile.tile_id: tile for tile in tiles}
    width = int(math.sqrt(len(tiles)))
    print(f'width: {width}')
    edge_to_tiles = defaultdict(list)
    for tile in tiles:
        # for side in tile.sides():
        for direction, side in tile.sides.items():
            clockwise_modification = Modification(direction, True)
            edge_to_tiles[side.clockwise].append((tile, clockwise_modification))
            counter_clockwise_modification = Modification(direction, False)
            edge_to_tiles[side.counter_clockwise].append((tile, counter_clockwise_modification))
    grid = [[top_left.tile_id]]
    top_row = [(top_left, Modification(Direction.TOP, True))]
    # fill top row
    for column_index in range(1, width):
        prev_tile, prev_mod = top_row[-1]
        prev_up_direction = prev_mod.direction
        prev_right_direction = prev_up_direction.next() if prev_mod.clockwise \
            else prev_up_direction.prev()
        # right_side = prev_tile.sides[prev_mod.direction.opposite()]
        right_side = prev_tile.sides[prev_right_direction]
        right_edge = right_side.clockwise if prev_mod.clockwise \
            else right_side.counter_clockwise
        possible_right_tiles = edge_to_tiles[right_edge]
        print(f'possible_right_tiles: {possible_right_tiles}')
        print(f'filtered: {[e for e in possible_right_tiles if e[0].tile_id != prev_tile.tile_id]}')
        right_tile, right_modification = [e for e in possible_right_tiles
                                               if e[0].tile_id != prev_tile.tile_id][0]
        top_row.append((right_tile, right_modification))
        # top_row.append(find_to_right(tile_id_to_tile, edge_to_tiles,
        #                              top_row[-1]))
    # fill other rows

    # for row_index in range(width):
    #     for column_index in range(width):
    #         if column_index == 0:
    for edge, tile_modifications in edge_to_tiles.items():
        print(edge)
        for tile, modification in tile_modifications:
            cw = 'CW' if modification.clockwise else 'CCW'
            print(f'\t{tile.tile_id}: {modification.direction} {cw}')
    pass

def find_to_right(tile_id_to_tile, edge_to_tiles, prev_tile_id):
    prev_tile = tile_id_to_tile[prev_tile_id]
    

def find_corner(tiles):
    for tile in tiles:
        if num_neighbors(tiles, tile) == 2:
            return tile

def num_neighbors(tiles, tile):
    total = 0
    for other_tile in tiles:
        if other_tile.tile_id == tile.tile_id:
            continue
        if tile.matches(other_tile):
            total += 1
    return total

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

@dataclass
class Side:
    clockwise: int
    counter_clockwise: int

    @staticmethod
    def create(string):
        clockwise = 0
        for c in string:
            clockwise *= 2
            if c == '#':
                clockwise += 1
        counter_clockwise = 0
        for c in reversed(string):
            counter_clockwise *= 2
            if c == '#':
                counter_clockwise += 1
        return Side(clockwise, counter_clockwise)

    def flip(self):
        return Side(self.counter_clockwise, self.clockwise)

    def matches(self, other_side):
        return self.clockwise == other_side.clockwise or \
            self.clockwise == other_side.counter_clockwise or \
            self.counter_clockwise == other_side.clockwise or \
            self.counter_clockwise == other_side.counter_clockwise

class Direction(Enum):
    TOP = auto()
    RIGHT = auto()
    BOTTOM = auto()
    LEFT = auto()

    def opposite(self):
        return add_wrapping_to_enum(self, 2)

    def to_index(self):
        return self.value - 1

    def next(self):
        return add_wrapping_to_enum(self, 1)

    def prev(self):
        return add_wrapping_to_enum(self, -1)

    @staticmethod
    def from_index(index):
        return Direction(index + 1)

    def __str__(self):
        return self.name

    def __repr__(self):
        return self.name

@dataclass
class Modification:
    direction: Direction
    clockwise: bool

@dataclass
class Tile:
    tile_id: str
    tile_lines: [str]
    sides: {Direction: Side}
    # top: int
    # right: int
    # bottom: int
    # left: int

    @staticmethod
    def create(tile_id, tile_lines):
        top = Side.create(tile_lines[0])
        bottom = Side.create(tile_lines[-1]).flip()
        left = Side.create([line[0] for line in tile_lines])
        right = Side.create([line[-1] for line in tile_lines]).flip()
        return Tile(tile_id, tile_lines, {
            Direction.TOP: top,
            Direction.RIGHT: right,
            Direction.BOTTOM: bottom,
            Direction.LEFT: left,
            
        })
        # return Tile(tile_id, top, right, bottom, left)

    def matches(self, other_tile):
        return any(any(side.matches(other_side)
                       # for other_side in other_tile.sides())
                       for other_side in other_tile.sides.values())
                   # for side in self.sides())
                   for side in self.sides.values())

    # def sides(self):
    #     return [self.top, self.right, self.bottom, self.left]
        
def product(iterable):
    total = 1
    for x in iterable:
        total *= x
    return total

def add_wrapping_to_enum(variant, add):
    cls = type(variant)
    zero_based_value = variant.value - 1
    zero_based_next_value = (zero_based_value + add) % len(cls)
    next_value = zero_based_next_value + 1
    return cls(next_value)

if __name__ == '__main__':
    main()
