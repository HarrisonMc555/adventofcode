#!/usr/bin/env python3

import re
from enum import Enum, auto

INPUT_FILE = 'input24.txt'
# INPUT_FILE = 'example24.txt'

NUM_DAYS = 100
# NUM_DAYS = 10

DEBUG = False

def main():
    text = get_text(INPUT_FILE)
    directions_list = parse_text(text)
    flipped = find_flipped_tiles(directions_list)
    debug_print(tiles_to_str(flipped))
    debug_print()
    for i in range(NUM_DAYS):
        # debug_print(f'-- Day {i + 1} --')
        # debug_print(f'\tflipped:')
        # for tile in sorted(flipped):
        #     debug_print(f'\t\t{tile}')
        # debug_print()
        # debug_print(tiles_to_str(flipped))
        # debug_print()
        flipped = next_day(flipped)
        if i < 10 or (i + 1) % 10 == 0:
            debug_print(f'Day {i + 1}: {len(flipped)}')
        if i == 10:
            debug_print()
    print(len(flipped))

def next_day(flipped):
    tiles = find_tiles_and_neighbors(flipped)
    # debug_print(f'\ttiles and neighbors:')
    # for tile in sorted(tiles):
    #     debug_print(f'\t\t{tile}')
    return set(tile for tile in tiles if flipped_next_day(tile, flipped))

def flipped_next_day(tile, flipped):
    flipped_neighbors = [t in flipped for t in neighbors_for_tile(tile)]
    num_flipped_neighbors = flipped_neighbors.count(True)
    if tile in flipped:
        return not (num_flipped_neighbors == 0 or num_flipped_neighbors > 2)
    else:
        return num_flipped_neighbors == 2

def find_tiles_and_neighbors(tiles):
    result = set(tiles)
    for tile in tiles:
        result.update(neighbors_for_tile(tile))
    return result

def neighbors_for_tile(tile):
    return [add_offset_to_position(tile, offset) for offset in DIRECTION_TO_OFFSET.values()]

def find_flipped_tiles(directions_list):
    flipped = set()
    for directions in directions_list:
        position = directions_to_position(directions)
        if position in flipped:
            flipped.remove(position)
        else:
            flipped.add(position)
    return flipped

def directions_to_position(directions):
    pos = 0, 0
    for direction in directions:
        offset = DIRECTION_TO_OFFSET[direction]
        pos = add_offset_to_position(pos, offset)
    return pos

def add_offset_to_position(position, offset):
    pos1, pos2 = position
    offset1, offset2 = offset
    return pos1 + offset1, pos2 + offset2

def parse_text(text):
    return [parse_line(line) for line in text.split('\n')]

DIRECTION_RE = re.compile(r'(e|se|sw|w|nw|ne)')
def parse_line(line):
    return [STR_TO_DIRECTION[s] for s in DIRECTION_RE.findall(line)]

class Direction(Enum):
    EAST = auto()
    SOUTH_EAST = auto()
    SOUTH_WEST = auto()
    WEST = auto()
    NORTH_WEST = auto()
    NORTH_EAST = auto()

STR_TO_DIRECTION = {
    'e': Direction.EAST,
    'se': Direction.SOUTH_EAST,
    'sw': Direction.SOUTH_WEST,
    'w': Direction.WEST,
    'ne': Direction.NORTH_EAST,
    'nw': Direction.NORTH_WEST,
}

DIRECTION_TO_STR = {d: s for s, d in STR_TO_DIRECTION.items()}

# First axis is east (positive) and west (negative)
# Second axis is southeast (positive) and northwest (negative)
DIRECTION_TO_OFFSET = {
    Direction.EAST: (1, 0),
    Direction.SOUTH_EAST: (0, 1),
    Direction.SOUTH_WEST: (-1, 1),
    Direction.WEST: (-1, 0),
    Direction.NORTH_EAST: (1, -1),
    Direction.NORTH_WEST: (0, -1),
}

def get_text(filename):
    with open(filename) as f:
        return f.read().strip()

def tiles_to_str(tiles):
    rows = [row for _, row in tiles]
    columns = [column for column, _ in tiles]
    min_row, max_row = min(rows), max(rows)
    min_column, max_column = min(columns), max(columns)
    lines = []
    for row in range(min_row, max_row + 1):
        line = []
        for column in range(min_column, max_column + 1):
            s = 'X' if (column, row) in tiles else '_'
            if (row, column) == (0, 0):
                s = 'O'
            line.append(s)
        prefix = ' ' * (row - min_row)
        lines.append(prefix + ' '.join(line))
    return '\n'.join(lines)

def debug_print(*args, **kwargs):
    if DEBUG:
        print(*args, **kwargs)

if __name__ == '__main__':
    main()
