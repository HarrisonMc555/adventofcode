#!/usr/bin/env python3

import unittest
import re

# TEST = True

# INPUT_FILE = 'example22a.txt'
# INPUT_FILE = 'example22b.txt'
INPUT_FILE = 'input22.txt'
MIN_VAL = -50
MAX_VAL = 50

DEBUG = True
def debug_print(*args, **kwargs):
    if 'DEBUG' in globals():
        print(*args, **kwargs)

def main():
    if 'TEST' in globals():
        unittest.main()
        return
    lines = get_lines(INPUT_FILE)
    commands = parse_lines(lines)
    state = run_commands(commands)
    # print(max(state))
    # print(min(state))
    print(len(state))

def run_commands(commands):
    state = set()
    for command in commands:
        run_command(state, command)
    return state

def run_command(state, command):
    on, nums = command
    x1, x2, y1, y2, z1, z2 = nums
    x1 = max(MIN_VAL, x1)
    x2 = min(MAX_VAL, x2)
    y1 = max(MIN_VAL, y1)
    y2 = min(MAX_VAL, y2)
    z1 = max(MIN_VAL, z1)
    z2 = min(MAX_VAL, z2)
    for x in range(x1, x2 + 1):
        for y in range(y1, y2 + 1):
            for z in range(z1, z2 + 1):
                cuboid = (x, y, z)
                if on:
                    state.add(cuboid)
                else:
                    state.discard(cuboid)

def parse_lines(lines):
    return [parse_line(line) for line in lines]

LINE_RE = re.compile(r'(on|off) x=(-?\d+)\.\.(-?\d+),y=(-?\d+)\.\.(-?\d+),z=(-?\d+)\.\.(-?\d+)')
def parse_line(line):
    match = LINE_RE.match(line)
    strings = list(match.groups())
    on_off = strings.pop(0)
    on = on_off == 'on'
    nums = [int(s) for s in strings]
    return on, nums

class Test(unittest.TestCase):
    def tests_parse_line(self):
        command = parse_line('on x=10..12,y=10..12,z=10..12')
        on, nums = command
        x1, x2, y1, y2, z1, z2 = nums
        self.assertTrue(on)
        self.assertEqual([10, 12, 10, 12, 10, 12], nums)

def get_lines(filename):
    with open(filename) as f:
        return [line.strip() for line in f.readlines() if line]

if __name__ == '__main__':
    main()
