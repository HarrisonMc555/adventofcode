#!/usr/bin/env python3

INPUT_FILE = 'input06.txt'

import unittest
import re

def main():
    if 'INPUT_FILE' not in globals():
        unittest.main()
        return
    lines = get_lines(INPUT_FILE)
    print(run(lines))

def run(lines):
    commands = parse_lines(lines)
    grid = [[0] * 1_000 for _ in range(1_000)]
    for command in commands:
        run_command(grid, command)
    return sum(sum(row) for row in grid)

def run_command(grid, command):
    which, (row1, col1, row2, col2) = command
    for row in range(row1, row2 + 1):
        for col in range(col1, col2 + 1):
            if which == 'turn on':
                diff = 1
            elif which == 'turn off':
                diff = -1
            elif which == 'toggle':
                diff = 2
            else:
                raise Exception(f'Unrecognized command: "{which}"')
            grid[row][col] += diff
            if grid[row][col] < 0:
                grid[row][col] = 0

def parse_lines(lines):
    return [parse_line(line.strip()) for line in lines]

LINE_RE = re.compile('(turn on|turn off|toggle) (\d+),(\d+) through (\d+),(\d+)')
def parse_line(line):
    try:
        groups = LINE_RE.match(line).groups()
        nums = [int(w) for w in groups[1:]]
        which = groups[0]
        return which, nums
    except Exception as e:
        print(f'Error parsing line "{line}"')
        raise e

def get_lines(filename):
    with open(filename) as f:
        return f.readlines()

EXAMPLE_LINES = [
    'turn on 0,0 through 999,999',
    'turn on 0,0 through 999,0',
    'turn on 499,499 through 500,500',
]
class Test(unittest.TestCase):
    def test_example(self):
        num_lit_lights = run(EXAMPLE_LINES)
        self.assertEqual(num_lit_lights, 998_996)

if __name__ == '__main__':
    main()
