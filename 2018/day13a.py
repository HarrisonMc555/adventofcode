#!/usr/bin/env python3
#pylint: disable=too-few-public-methods, invalid-name

import sys
from enum import Enum, auto

class CartException(Exception):
    pass

class Direction(Enum):
    UP = auto()
    DOWN = auto()
    LEFT = auto()
    RIGHT = auto()

    def get_drow_dcol(self):
        if self == Direction.UP:
            return -1, 0
        if self == Direction.DOWN:
            return 1, 0
        if self == Direction.LEFT:
            return 0, -1
        if self == Direction.RIGHT:
            return 0, 1
        raise Exception('Invalid direction')

    def next_from_tile(self, tile):
        if tile in [Tile.VERTICAL, Tile.HORIZONTAL]:
            return self
        if tile == Tile.TURN_UP_TO_RIGHT:
            return self.turn_clockwise() if self.is_vertical() else \
                self.turn_counterclockwise()
        if tile == Tile.TURN_UP_TO_LEFT:
            return self.turn_counterclockwise() if self.is_vertical() else \
                self.turn_clockwise()
        if tile == Tile.EMPTY:
            raise Exception('Traveling on empty tile')
        raise Exception('Invalid tile', tile)

    def turn_clockwise(self):
        if self == Direction.UP:
            return Direction.RIGHT
        if self == Direction.RIGHT:
            return Direction.DOWN
        if self == Direction.DOWN:
            return Direction.LEFT
        if self == Direction.LEFT:
            return Direction.UP
        raise Exception('Invalid direction')

    def turn_counterclockwise(self):
        if self == Direction.UP:
            return Direction.LEFT
        if self == Direction.LEFT:
            return Direction.DOWN
        if self == Direction.DOWN:
            return Direction.RIGHT
        if self == Direction.RIGHT:
            return Direction.UP
        raise Exception('Invalid direction')

    def turn(self, turn):
        pass

    def is_vertical(self):
        return self in [Direction.UP, Direction.DOWN]

    def is_horizontal(self):
        return self in [Direction.LEFT, Direction.RIGHT]

class Cart:
    TURNS = [Direction.turn_counterclockwise,
             lambda d: d,
             Direction.turn_clockwise]

    def __init__(self, row, col, direction):
        self.row = row
        self.col = col
        self.direction = direction
        self.turn_index = 0

    def get_tile_underneath(self):
        d = self.direction
        if d in [Direction.UP, Direction.DOWN]:
            return Tile.VERTICAL
        if d in [Direction.LEFT, Direction.RIGHT]:
            return Tile.HORIZONTAL
        raise CartException('Invalid direction')

    def get_location(self):
        return self.row, self.col

    def tick(self, grid):
        drow, dcol = self.direction.get_drow_dcol()
        self.row += drow
        self.col += dcol
        tile = grid[self.row][self.col]
        if tile == Tile.INTERSECTION:
            turn_function = Cart.TURNS[self.turn_index]
            self.direction = turn_function(self.direction)
            self.turn_index = (self.turn_index + 1) % len(Cart.TURNS)
        else:
            self.direction = self.direction.next_from_tile(tile)

    def to_char(self):
        if self.direction == Direction.UP:
            return '^'
        if self.direction == Direction.DOWN:
            return 'v'
        if self.direction == Direction.LEFT:
            return '<'
        if self.direction == Direction.RIGHT:
            return '>'
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

class Tile(Enum):
    VERTICAL = auto()
    HORIZONTAL = auto()
    TURN_UP_TO_RIGHT = auto()
    TURN_UP_TO_LEFT = auto()
    INTERSECTION = auto()
    EMPTY = auto()

    def to_char(self):
        if self == Tile.VERTICAL:
            return '|'
        if self == Tile.HORIZONTAL:
            return '-'
        if self == Tile.TURN_UP_TO_RIGHT:
            return '/'
        if self == Tile.TURN_UP_TO_LEFT:
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
            return Tile.TURN_UP_TO_RIGHT
        if char == '\\':
            return Tile.TURN_UP_TO_LEFT
        if char == '+':
            return Tile.INTERSECTION
        if char == ' ':
            return Tile.EMPTY
        raise Exception('Invalid tile char')

def solve(carts, grid):
    collision = get_first_collision(carts, grid)
    row, col = collision
    x, y = col, row
    return x, y

def get_first_collision(carts, grid):
    # print_state(carts, grid)
    while True:
        collision = tick_all(carts, grid)
        if collision:
            return collision
        # print_state(carts, grid)
    return None

def print_state(carts, grid):
    print('carts:')
    for cart in carts:
        print(cart)
    print()
    print('grid:')
    cart_dict = {cart.get_location(): cart.to_char() for cart in carts}
    for i, row in enumerate(grid):
        line = []
        for j, tile in enumerate(row):
            location = i, j
            char = cart_dict.get(location, tile.to_char())
            line.append(char)
        print(''.join(line))

def tick_all(carts, grid):
    for cart in carts:
        cart.tick(grid)
        collision = get_collision(cart, carts)
        if collision:
            return collision
    return None

def get_collision(cart, carts):
    location = cart.get_location()
    for other_cart in carts:
        if cart is other_cart:
            continue
        if location == other_cart.get_location():
            return location
    return None

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
    x, y = solve(carts, grid)
    print('{},{}'.format(x, y))

if __name__ == '__main__':
    main()
