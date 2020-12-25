#!/usr/bin/env python3

from enum import Enum, auto
from dataclasses import dataclass
import re

# INPUT_FILE = 'input20.txt'
INPUT_FILE = 'example20.txt'

def main():
    text = get_text(INPUT_FILE)
    tiles = parse_text(text)
    for tile_id, tile in tiles.items():
        print(f'{tile_id}: {tile}')

# For every tile
def assemble_grid(tiles):
    connections = find_connections(tiles)
    pass

def find_connections(tiles):
    pass

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

    def to_index(self):
        return self.value - 1

@dataclass
class Connection:
    tile_id: str
    side: Side
    other_tile_id: str
    other_side: Side
    flipped: bool

@dataclass
class Modification:
    flip_horizontal: bool
    flip_vertical: bool
    rotations: int

if __name__ == '__main__':
    main()
