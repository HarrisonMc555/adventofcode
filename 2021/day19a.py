#!/usr/bin/env python3

INPUT_FILE = 'input19.txt'
INPUT_FILE = 'example19.txt'

import unittest
import re
from enum import Enum, auto

DEBUG = False
# DEBUG = True
def debug_print(*args, **kwargs):
    if DEBUG:
        print(*args, **kwargs)

def main():
    if 'INPUT_FILE' not in globals():
        unittest.main()
        return
    scanners = parse_text(get_text(INPUT_FILE))
    print(run(scanners))
    print(Axis.get_remaining_axis([Axis.X, Axis.Y]))

def run(scanners):
    for scanner_index, beacons in scanners:
        print(f'scanner_index: {scanner_index}')
        for beacon in beacons:
            print(f'beacon: {beacon}')
        print()

class Axis(Enum):
    X = auto()
    Y = auto()
    Z = auto()

    axis_to_bits = {
        X: 0b001,
        Y: 0b010,
        Z: 0b100,
    }

    bits_to_axis = {
        0b001: X,
        0b010: Y,
        0b100: Z,
    }

    all_bits = 0b111

    def to_bits(self):
        return Axis.axis_to_bits[self]

    @staticmethod
    def get_remaining_axis(axes):
        bits = Axis.all_bits
        for axis in axes:
            bits &= ~axis.to_bits()
        return Axis.get_axis_from_bits(bits)

    @staticmethod
    def get_axis_from_bits(bits):
        return bits_to_axis[bits]

class Direction(Enum):
    XPos = auto()
    XNeg = auto()
    YPos = auto()
    YNeg = auto()
    ZPos = auto()
    ZNeg = auto()

    dir_to_axis = {
        XPos: Axis.X,
        XNeg: Axis.X,
        YPos: Axis.Y,
        YNeg: Axis.Y,
        ZPos: Axis.Z,
        ZNeg: Axis.Z,
    }
    
    dir_to_positive = {
        XPos: True,
        XNeg: False,
        YPos: True,
        YNeg: False,
        ZPos: True,
        ZNeg: False,
    }

    dir_to_index = {
        XPos: 0,
        XNeg: 0,
        YPos: 1,
        YNeg: 1,
        ZPos: 2,
        ZNeg: 2,
    }

    def axis(self):
        return dir_to_axis[self]

    def positive(self):
        return dir_to_positive[self]

    def multiplier(self):
        return 1 if self.positive() else -1

    def index(self):
        return dir_to_index[self]

class Modification:
    forward: Direction
    up: Direction

    def __init__(self, forward, up):
        assert forward.axis() != up.axis()
        self.forward = forward
        self.up = up

    def modify(self, coords):
        x, y, z = coords
        result = [None] * 3
        result[0] = coords[self.forward.index] * self.forward.multiplier()
        result[1] = coords[self.up.index] * self.up.multiplier()
        # last_axis = s
        # result[2] = coords[0]
        # self.forward.
        for i in range(3):
            self
        
        return tuple(result)

######################################################################

def parse_text(text):
    return [parse_group(group) for group in text.split('\n\n')]

HEADER_PATTERN = re.compile(r'--- scanner (\d+) ---')
def parse_group(text):
    lines = text.strip().split('\n')
    header_string = lines[0]
    scanner_index = int(HEADER_PATTERN.match(header_string).groups()[0])
    beacon_lines = lines[1:]
    beacons = [parse_beacon_line(line) for line in beacon_lines]
    return scanner_index, beacons

def parse_beacon_line(line):
    return tuple(int(w) for w in line.split(','))

def get_text(filename):
    with open(filename) as f:
        return f.read().strip()

class Test(unittest.TestCase):
    pass

if __name__ == '__main__':
    main()
