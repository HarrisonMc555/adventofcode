#!/usr/bin/env python3
#pylint: disable=too-few-public-methods, invalid-name

import sys
from enum import Enum, auto

class CartException(Exception):
    pass

class Cart:
    def __init__(self, row, col, direction):
        self.row = row
        self.col = col
        self.direction = direction

    def get_tile_underneath(self):
        d = self.direction
        if d in [Direction.UP, Direction.DOWN]:
            return Tile.VERTICAL
        if d in [Direction.LEFT, Direction.RIGHT]:
            return Tile.HORIZONTAL
        raise CartException('Invalid direction')

    def __repr__(self):
        return 'Cart({}, {}, {})'.format(self.row, self.col,
                                         str(self.direction))

    @staticmethod
    def from_char(row, col, char):
        return Cart(row, col, Cart.direction_from_char(char))

    @staticmethod
    def direction_from_char(char):
        if char == '^':
            return Direction.UP
        if char == 'v':
            return Direction.DOWN
        if char == '<':
            return Direction.LEFT
        if char == '>':
            return Direction.RIGHT
        raise CartException('Invalid direction char')

    @staticmethod
    def is_cart_char(char):
        try:
            Cart.direction_from_char(char)
            return True
        except CartException:
            return False

class Direction(Enum):
    UP = auto()
    DOWN = auto()
    LEFT = auto()
    RIGHT = auto()

    def get_didj(self):
        if self == Direction.UP:
            return -1, 0
        if self == Direction.DOWN:
            return 1, 0
        if self == Direction.LEFT:
            return 0, -1
        if self == Direction.RIGHT:
            return 0, 1
        raise Exception('Invalid direction')

class Tile(Enum):
    VERTICAL = auto()
    HORIZONTAL = auto()
    TURN_UP_RIGHT = auto()
    TURN_DOWN_LEFT = auto()
    INTERSECTION = auto()
    EMPTY = auto()

    def to_char(self):
        if self == Tile.VERTICAL:
            return '|'
        if self == Tile.HORIZONTAL:
            return '-'
        if self == Tile.TURN_UP_RIGHT:
            return '/'
        if self == Tile.TURN_DOWN_LEFT:
            return '\\'
        if self == Tile.INTERSECTION:
            return '+'
        if self == Tile.EMPTY:
            return ' '
        raise Exception('Invalid Tile')

    @staticmethod
    def from_char(char):
        if char == '|':
            return Tile.VERTICAL
        if char == '-':
            return Tile.HORIZONTAL
        if char == '/':
            return Tile.TURN_UP_RIGHT
        if char == '\\':
            return Tile.TURN_DOWN_LEFT
        if char == '+':
            return Tile.INTERSECTION
        if char == ' ':
            return Tile.EMPTY
        raise Exception('Invalid tile char')

def solve(carts, grid):
    print('carts:')
    for cart in carts:
        print(cart)
    print()
    print('grid:')
    for row in grid:
        print(''.join(tile.to_char() for tile in row))
    return carts, grid

def get_input():
    carts = []
    grid = []
    for row, line in enumerate(get_lines()):
        line_carts, tracks = parse_line(row, line)
        carts.extend(line_carts)
        grid.append(tracks)
    return carts, grid

def get_lines():
    return [line.strip('\n') for line in sys.stdin.readlines()]

def parse_line(row, line):
    carts = []
    tracks = []
    for col, char in enumerate(line):
        if Cart.is_cart_char(char):
            cart = Cart.from_char(row, col, char)
            carts.append(cart)
            tracks.append(cart.get_tile_underneath())
        else:
            tracks.append(Tile.from_char(char))
    return carts, tracks

def main():
    carts, grid = get_input()
    solve(carts, grid)
    # print(solve(lines))

if __name__ == '__main__':
    main()
