#!/usr/bin/env python3

import re
from enum import Enum, auto

INPUT_FILE = 'input24.txt'
# INPUT_FILE = 'example24.txt'

DEBUG = True

def main():
    text = get_text(INPUT_FILE)
    directions_list = parse_text(text)
    flipped = find_flipped_tiles(directions_list)
    print(len(flipped))

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
    pos1, pos2 = 0, 0
    for direction in directions:
        diff1, diff2 = DIRECTION_TO_OFFSET[direction]
        pos1 += diff1
        pos2 += diff2
    return pos1, pos2

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

def debug_print(*args, **kwargs):
    if DEBUG:
        print(*args, **kwargs)

if __name__ == '__main__':
    main()
