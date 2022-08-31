#!/usr/bin/env python3

INPUT_FILE = 'input25.txt'

import unittest
import re
import itertools
from tabulate import tabulate

DEBUG = True
DEBUG = False


def debug_print(*args, **kwargs):
    if DEBUG:
        print(*args, **kwargs)


def main():
    if 'INPUT_FILE' not in globals():
        unittest.main()
        return
    row, column = parse_text(get_text(INPUT_FILE))
    print(run(row, column))


def run(row, column):
    num = get_num(row, column)
    state = 20151125
    for _ in range(num - 1):
        state = next_state(state)
    return state


def next_state(state):
    return (state * 252533) % 33554393


def get_num(row, column):
    diagonal = row + column - 1
    prev_sum = sum(range(diagonal))
    return prev_sum + column


PATTERN = re.compile(
    r'To continue, please consult the code grid in the manual.  Enter the code at row (\d+), column (\d+).')


def parse_text(text):
    row_s, column_s = PATTERN.match(text).groups()
    return int(row_s), int(column_s)


def get_text(filename):
    with open(filename) as f:
        return f.read()


class Test(unittest.TestCase):
    def test_get_num(self):
        table = [
            [1, 3, 6, 10, 15, 21],
            [2, 5, 9, 14, 20],
            [4, 8, 13, 19],
            [7, 12, 18],
            [11, 17],
            [16]
        ]
        self.validate_table(table, get_num)

    def test_example(self):
        table = [
            [20151125, 18749137, 17289845, 30943339, 10071777, 33511524],
            [31916031, 21629792, 16929656, 7726640, 15514188, 4041754],
            [16080970, 8057251, 1601130, 7981243, 11661866, 16474243],
            [24592653, 32451966, 21345942, 9380097, 10600672, 31527494],
            [77061, 17552253, 28094349, 6899651, 9250759, 31663883],
            [33071741, 6796745, 25397450, 24659492, 1534922, 27995004],
        ]
        self.validate_table(table, run)

    def validate_table(self, table, f):
        for ri, row in enumerate(table):
            ri += 1
            for ci, num in enumerate(row):
                ci += 1
                self.assertEqual(num, f(ri, ci),
                                 f'Row {ri} and column {ci}')
        

if __name__ == '__main__':
    main()
