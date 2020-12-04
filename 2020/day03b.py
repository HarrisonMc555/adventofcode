#!/usr/bin/env python3

from enum import Enum, auto

INPUT_FILE = 'input03.txt'
SLOPES = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]

def main():
    lines = get_lines_from_file(INPUT_FILE)
    tree_map = parse_map(lines)
    print(all_trees_product(tree_map, SLOPES))

def all_trees_product(tree_map, slopes):
    num_trees = [count_trees(tree_map, right, down) for (right, down) in slopes]
    return product(num_trees)

def get_lines_from_file(filename):
    with open(filename) as f:
        return [line.strip() for line in f.readlines()]

def parse_map(lines):
    return [parse_line(line) for line in lines]

def parse_line(line):
    return [Tile.from_char(c) for c in line]

class Tile(Enum):
    TREE = auto()
    OPEN = auto()

    def from_char(c):
        if c == '#':
            return Tile.TREE
        elif c == '.':
            return Tile.OPEN
        else:
            raise Exception(f'Invalid character {c}')

def count_trees(tree_map, slope_right, slope_down):
    row, column = 0, 0
    num_trees = 0
    num_columns = len(tree_map[0])
    while row < len(tree_map):
        if tree_map[row][column] == Tile.TREE:
            num_trees += 1
        column = (column + slope_right) % num_columns
        row += slope_down
    return num_trees

def product(nums):
    result = 1
    for num in nums:
        result *= num
    return result

if __name__ == '__main__':
    main()
