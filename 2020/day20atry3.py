#!/usr/bin/env python3

import math
from enum import Enum, auto
from dataclasses import dataclass
import re

INPUT_FILE = 'input20.txt'
# INPUT_FILE = 'example20.txt'

def main():
    text = get_text(INPUT_FILE)
    tiles = parse_text(text)
    # for tile_id, tile in tiles.items():
    #     print(f'{tile_id}: {tile}')
    # print()
    grid = assemble_grid(tiles)
    if not grid:
        print('No solution found')
        return
    # for row in grid:
    #     print(' '.join(str(x) for x in row))
    # print()

    corners = [
        grid[0][0],
        grid[0][-1],
        grid[-1][0],
        grid[-1][-1],
    ]

    print(product(corners))

# For every tile
def assemble_grid(tiles):
    tile_id_to_connections = find_connections(tiles)
    # for tile_id, connections in tile_id_to_connections.items():
    #     print(tile_id)
    #     for connection in connections:
    #         print(f'\t{connection}')
    num_rows = int(math.sqrt(len(tiles)))
    num_cols = num_rows
    grid = [[None] * num_cols for _ in range(num_rows)]
    tile_ids = set(tiles)
    grid = GridAssembler(tiles, tile_id_to_connections).assemble()
    return [[tile_id for tile_id, _ in row] for row in grid]
    # return assemble_grid_helper(tiles, tile_id_to_connections, tile_ids, grid, 0, 0, num_rows, num_cols)

class GridAssembler:
    def __init__(self, tiles, tile_id_to_connections):
        self.tiles = tiles
        self.tile_id_to_connections = tile_id_to_connections
        self.row_i = 0
        self.col_i = 0
        self.num_rows = int(math.sqrt(len(tiles)))
        self.num_cols = self.num_rows
        self.grid = [[None] * self.num_cols for _ in range(self.num_rows)]

    def assemble(self):
        return self.assemble_helper(0, 0, set(self.tiles))

    def assemble_helper2(self, row_i, col_i, tile_ids):
        if col_i >= self.num_cols:
            row_i += 1
            col_i = 0
        if row_i >= self.num_rows:
            return self.grid

        for tile_id in tile_ids:
            tile = self.tiles[tile_id]

            for connection in self.tile_id_to_connections[tile_id]

    def assemble_helper(self, row_i, col_i, tile_ids):
        if col_i >= self.num_cols:
            row_i += 1
            col_i = 0
        if row_i >= self.num_rows:
            return self.grid

        offset = ' ' * (row_i * self.num_cols + col_i)
        # print(f'*', end='')
        # # print(grid)
        # for row in self.grid:
        #     # print(offset + ' '.join(str(tup[0]) if tup else '-' for tup in row))
        #     print(offset + ' '.join(elem_to_str(elem) for elem in row))
        # print()
    
        for tile_id in tile_ids:
            tile = self.tiles[tile_id]
            for modification in ALL_MODIFICATIONS:
                if col_i > 0:
                    other_tile_id, other_modification = self.grid[row_i][col_i - 1]
                    other_tile = self.tiles[other_tile_id]
                    if not tile.matches(modification, other_tile, other_modification, Side.LEFT):
                        continue
                
                if row_i > 0:
                    other_tile_id, other_modification = self.grid[row_i - 1][col_i]
                    other_tile = self.tiles[other_tile_id]
                    if not tile.matches(modification, other_tile, other_modification, Side.TOP):
                        continue
    
                self.grid[row_i][col_i] = tile_id, modification
                result = self.assemble_helper(row_i, col_i + 1, tile_ids.difference([tile_id]))
                if result:
                    return result
                self.grid[row_i][col_i] = None
        return None


def elem_to_str(elem):
    if not elem:
        return '-' * 8
    tile_id, modification = elem
    mod_s = ''.join([
        'T' if modification.flip_horizontal else 'F',
        'T' if modification.flip_vertical else 'F',
        str(modification.rotations)
    ])
    return f'{tile_id}-{mod_s}'

def find_connections(tiles):
    return {tile_id: create_connections(tile_id, tiles) for tile_id in tiles}

def create_connections(tile_id, tiles):
    tile = tiles[tile_id]
    connections = []
    for i, num in enumerate(tile.cw_nums):
        side = Side.from_index(i)
        for other_tile_id, other_tile in tiles.items():
            if other_tile.tile_id == tile_id:
                continue

            for other_i, other_num in enumerate(other_tile.cw_nums):
                other_side = Side.from_index(other_i)
                if other_num == num:
                    connection = Connection(tile_id, side, other_tile_id, other_side, True)
                    connections.append(connection)

            for other_i, other_num in enumerate(other_tile.ccw_nums):
                other_side = Side.from_index(other_i)
                if other_num == num:
                    connection = Connection(tile_id, side, other_tile_id, other_side, False)
                    connections.append(connection)
                    
    for i, num in enumerate(tile.ccw_nums):
        side = Side.from_index(i)
        for other_tile_id, other_tile in tiles.items():
            if other_tile.tile_id == tile_id:
                continue

            for other_i, other_num in enumerate(other_tile.cw_nums):
                other_side = Side.from_index(other_i)
                if other_num == num:
                    connection = Connection(tile_id, side, other_tile_id, other_side, False)
                    connections.append(connection)

            for other_i, other_num in enumerate(other_tile.ccw_nums):
                other_side = Side.from_index(other_i)
                if other_num == num:
                    connection = Connection(tile_id, side, other_tile_id, other_side, True)
                    connections.append(connection)
                    
    return connections

def edges_match(tiles, tile_id1, mod1, side1, tile_id2, mod2, side2):
    tile1 = tiles[tile_id1]
    tile2 = tiles[tile_id2]
    edge_num1 = tile1.get_edge_num(mod1, side1, False)
    edge_num2 = tile2.get_edge_num(mod2, side2, True)
    return edge_num1 == edge_num2

def get_text(filename):
    with open(filename) as f:
        return f.read().strip()

def parse_text(text):
    tiles_texts = text.split('\n\n')
    tiles = [parse_tile(t) for t in tiles_texts]
    return {tile.tile_id: tile for tile in tiles}

HEADER_RE = re.compile('Tile (\d+):')
def parse_tile(text):
    lines = text.split('\n')
    header = lines[0]
    tile_id = int(HEADER_RE.match(header).group(1))
    lines = lines[1:]
    grid = [list(row) for row in lines]
    edges = get_edges(grid)
    cw_nums = [edge_to_num(edge) for edge in edges]
    ccw_nums = [edge_to_num(list(reversed(edge))) for edge in edges]
    return Tile(tile_id, cw_nums, ccw_nums)

def get_edges(grid):
    right = [row[-1] for row in grid]
    bottom = list(reversed(grid[-1]))
    left = [row[0] for row in reversed(grid)]
    top = grid[0]
    return [right, bottom, left, top]

BASE = 2
def edge_to_num(edge):
    num_digits = len(edge)
    num = 0
    factor = 1
    for c in reversed(edge):
        if c == '#':
            num += factor
        factor *= BASE
    return num

@dataclass
class Tile:
    tile_id: str
    cw_nums: [int]
    ccw_nums: [int]

    def matches(self, modification, other_tile, other_modification, side):
        side_num = self.get_edge_num(modification, side, False)
        other_side_num = other_tile.get_edge_num(other_modification, side.opposite(), True)
        return side_num == other_side_num

    def get_edge_num(self, modification, side, flipped):
        rotations = modification.rotations
        index = side.to_index()
        factor = 1
        if modification.flip_horizontal and modification.flip_vertical:
            rotations = (rotations + 2) % 4
        elif modification.flip_horizontal:
            factor = -1
            rotations += 2
            flipped = not flipped
        elif modification.flip_vertical:
            factor = -1
            rotations += 0
            flipped = not flipped
        index = (side.to_index() - rotations) % 4
        index = (index * factor) % 4
        nums = self.ccw_nums if flipped else self.cw_nums
        return nums[index]


'''
original:
123
456
789

  right:  369 (0 F)
  bottom: 987 (1 F)
  left:   741 (2 F)
  top:    123 (3 F)


rotate: 1
741
852
963

  right:  123 (3 F)
  bottom: 369 (0 F)
  left:   987 (1 F)
  top:    741 (2 F)

rotate: 2
987
654
321

  right:  741 (2 F)
  bottom: 123 (3 F)
  left:   369 (0 F)
  top:    987 (1 F)

rotate: 3
369
258
147

  right:  987 (1 F)
  bottom: 741 (2 F)
  left:   123 (3 F)
  top:    369 (0 F)


flip_h:
321
654
987

  right:  147 (2 T)
  bottom: 789 (1 T)
  left:   963 (0 T)
  top:    321 (3 T)

flip_v:
789
456
123

  right:  963 (0 T)
  bottom: 321 (3 T)
  left:   147 (2 T)
  top:    789 (1 T)
'''
        
class Side(Enum):
    RIGHT = auto()
    BOTTOM = auto()
    LEFT = auto()
    TOP = auto()

    def opposite(self):
        return add_wrapping_to_enum(self, 2)

    def to_index(self):
        return self.value - 1

    @staticmethod
    def from_index(index):
        return Side(index + 1)

    def __str__(self):
        return self.name

    def __repr__(self):
        return self.name

@dataclass
class Connection:
    tile_id: int
    side: Side
    other_tile_id: int
    other_side: Side
    flipped: bool

@dataclass
class Modification:
    flip_horizontal: bool
    flip_vertical: bool
    rotations: int

    @staticmethod
    def create_all():
        modifications = []
        for flip_horizontal in [False, True]:
            for flip_vertical in [False, True]:
                for rotations in range(4):
                    modifications.append(Modification(flip_horizontal, flip_vertical, rotations))
        return modifications

ALL_MODIFICATIONS = Modification.create_all()

def add_wrapping_to_enum(variant, add):
    cls = type(variant)
    zero_based_value = variant.value - 1
    zero_based_next_value = (zero_based_value + add) % len(cls)
    next_value = zero_based_next_value + 1
    return cls(next_value)
    
def product(nums):
    result = 1
    for num in nums:
        result *= num
    return result

if __name__ == '__main__':
    main()
