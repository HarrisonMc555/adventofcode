#!/usr/bin/env python3

from enum import Enum, auto

INPUT_FILE = 'input12.txt'
# INPUT_FILE = 'example.txt'

def main():
    lines = get_lines(INPUT_FILE)
    commands = parse_lines(lines)
    boat = Boat(Direction.EAST, Position(0, 0))
    for operation, value in commands:
        # print(boat)
        # print(f'\tExecuting command {operation.name}, {value}')
        boat = boat.run_command(operation, value)
    # print(boat)
    x, y = boat.position.x, boat.position.y
    print(abs(x) + abs(y))

def get_lines(filename):
    with open(filename) as f:
        return [line.strip() for line in f.readlines() if line]

def parse_lines(lines):
    return [parse_line(line) for line in lines]

def parse_line(line):
    operation = CHAR_TO_OPERATION[line[0]]
    value = int(line[1:])
    return operation, value

class Operation(Enum):
    NORTH = auto()
    EAST = auto()
    SOUTH = auto()
    WEST = auto()
    FORWARD = auto()
    RIGHT = auto()
    LEFT = auto()

    def __str__(self):
        return OPERATION_TO_CHAR[self]

    def from_char(c):
        return CHAR_TO_OPERATION[c]

OPERATION_TO_CHAR = {
    Operation.NORTH: 'N',
    Operation.EAST: 'E',
    Operation.SOUTH: 'S',
    Operation.WEST: 'W',
    Operation.FORWARD: 'F',
    Operation.RIGHT: 'R',
    Operation.LEFT: 'L',
}

CHAR_TO_OPERATION = {
    'N': Operation.NORTH,
    'E': Operation.EAST,
    'S': Operation.SOUTH,
    'W': Operation.WEST,
    'F': Operation.FORWARD,
    'R': Operation.RIGHT,
    'L': Operation.LEFT,
}

class Direction(Enum):
    NORTH = auto()
    EAST = auto()
    SOUTH = auto()
    WEST = auto()

    def turn(self, degrees):
        increments = degrees // 90
        return add_wrapping_to_enum(self, increments)

def add_wrapping_to_enum(variant, add):
    cls = type(variant)
    zero_based_value = variant.value - 1
    zero_based_next_value = (zero_based_value + add) % len(cls)
    next_value = zero_based_next_value + 1
    return cls(next_value)
    
class Position:
    X_FACTOR = {
        Direction.NORTH: 0,
        Direction.EAST: 1,
        Direction.SOUTH: 0,
        Direction.WEST: -1,
    }

    Y_FACTOR = {
        Direction.NORTH: -1,
        Direction.EAST: 0,
        Direction.SOUTH: 1,
        Direction.WEST: 0,
    }

    def __init__(self, x, y):
        self.x = x
        self.y = y

    def move(self, direction, value):
        # print(f'\t\tCurrent position: ({self.x}, {self.y}).')
        # print(f'\t\tDirection: {direction.name}.')
        # print(f'\t\tValue: {value}.')
        # print(f'\t\tX factor: {Position.X_FACTOR[direction]}.')
        # print(f'\t\tY factor: {Position.Y_FACTOR[direction]}.')
        x = self.x + Position.X_FACTOR[direction] * value
        y = self.y + Position.Y_FACTOR[direction] * value
        return Position(x, y)


class Boat:
    def __init__(self, direction, position):
        self.direction = direction
        self.position = position

    def run_command(self, operation, value):
        position = self.position
        direction = self.direction
        abs_direction = OPERATION_TO_DIRECTION.get(operation, None)
        if abs_direction:
            position = position.move(abs_direction, value)
        elif operation == Operation.FORWARD:
            position = position.move(self.direction, value)
        else:
            direction = direction.turn(value * TURN_FACTOR[operation])
        return Boat(direction, position)

    def __str__(self):
        return f'Boat going {self.direction.name} at ({self.position.x}, {self.position.y})'

OPERATION_TO_DIRECTION = {
    Operation.NORTH: Direction.NORTH,
    Operation.EAST: Direction.EAST,
    Operation.SOUTH: Direction.SOUTH,
    Operation.WEST: Direction.WEST,
}

TURN_FACTOR = {
    Operation.RIGHT: 1,
    Operation.LEFT: -1,
}


if __name__ == '__main__':
    main()
