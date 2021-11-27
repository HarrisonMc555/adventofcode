#!/usr/bin/env python3

import re
from dataclasses import dataclass

INPUT_FILE = 'input20.txt'
# INPUT_FILE = 'example20.txt'

TILE_RE = re.compile('Tile (\d+):')

def main():
    text = get_text(INPUT_FILE)
    tile_dict = parse_tiles(text)
    tile = list(tile_dict.values())[0]
    num = num_neighbors(tile_dict.values(), tile)
    corners = [tile.tile_id for tile in tile_dict.values()
               if num_neighbors(tile_dict.values(), tile) < 4]
    # print(corners)
    print(product(corners))

def num_neighbors(tiles, tile):
    total = 0
    for other_tile in tiles:
        if tile.matches(other_tile):
            total += 1
    return total

def get_text(filename):
    with open(filename) as f:
        return f.read().strip()

def parse_tiles(text):
    tiles = [parse_tile(tile) for tile in text.split('\n\n')]
    return {tile.tile_id: tile for tile in tiles}

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

@dataclass
class Tile:
    tile_id: str
    top: str
    right: str
    bottom: str
    left: str

    @staticmethod
    def create(tile_id, tile_lines):
        top = Side.create(tile_lines[0])
        bottom = Side.create(tile_lines[-1]).flip()
        left = Side.create([line[0] for line in tile_lines])
        right = Side.create([line[-1] for line in tile_lines]).flip()
        return Tile(tile_id, top, right, bottom, left)

    def matches(self, other_tile):
        return any(any(side.matches(other_side)
                       for other_side in other_tile.sides())
                   for side in self.sides())

    def sides(self):
        return [self.top, self.right, self.bottom, self.left]
        
def product(iterable):
    total = 1
    for x in iterable:
        total *= x
    return total

if __name__ == '__main__':
    main()
