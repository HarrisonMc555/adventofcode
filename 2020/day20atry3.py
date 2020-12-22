#!/usr/bin/env python3

import re

INPUT_FILE = 'input20.txt'
# INPUT_FILE = 'example20.txt'

def main():
    text = get_text(INPUT_FILE)
    tiles = parse_text(text)

# For every tile
def assemble_grid(tiles):
    connections = find_connections(tiles)
    pass

def find_connections(tiles):
    pass

def edges_match(tiles, tile_id1, mod1, side1, tile_id2, mod2, side2):
    edge1 = get_edge(tiles[tile_id1], mod1, side1)
    edge2 = get_edge(tiles[tile_id2], mod2, side2)
    edge2.reverse()
    return edge1 == edge2

class Connection:
    def __init__(self, tile_id, side, other_tile_id, other_side, flipped):
        self.tile_id = tile_id
        self.side = side
        self.other_tile_id = other_tile_id
        self.other_side = other_side
        self.flipped = flipped

class Modification:
    def __init__(self, flip_horizontal, flip_vertical, rotations):
        self.flip_horizontal = flip_horizontal
        self.flip_vertical = flip_vertical
        self.rotations = self.rotations

def get_text(filename):
    with open(filename) as f:
        return f.read().strip()

def parse_text(text):
    tiles_texts = text.split('\n\n')
    return dict(parse_tile(t) for t in tiles_texts)

HEADER_RE = re.compile('Tile (\d+):')
def parse_tile(text):
    lines = text.split('\n')
    header = lines[0]
    tile_id = int(HEADER_RE.match(header).group(1))
    lines = lines[1:]
    grid = [list(row) for row in lines]
    return tile_id, grid

if __name__ == '__main__':
    main()
