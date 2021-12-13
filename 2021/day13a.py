#!/usr/bin/env python3

INPUT_FILE = 'input13.txt'
# INPUT_FILE = 'example13.txt'

import unittest
import re

DEBUG = False
# DEBUG = True
def debug_print(*args, **kwargs):
    if DEBUG:
        print(*args, **kwargs)

NUM_SECONDS = 2503
def main():
    text = get_text(INPUT_FILE)
    coords, commands = parse_text(text)
    grid = create_grid_from_coords(coords)
    print_grid(grid)
    for command in commands:
        grid = run_command(grid, command)
        print_grid(grid)
        # Quit after first command
        break
    print(sum(1 for row in grid for cell in row if cell))

def run_command(grid, command):
    xy, amount = command
    debug_print(f'Running command: fold along {xy} = {amount}')
    if xy == 'x':
        num_rows = len(grid)
        num_cols = amount
    elif xy == 'y':
        num_rows = amount
        num_cols = len(grid[0])
    else:
        raise Exception(f'Unrecognized xy: {xy}')
    new_grid = create_grid(num_rows, num_cols)
    for y in range(len(grid)):
        if xy == 'y' and y > amount:
            new_y = 2 * amount - y
        else:
            new_y = y
        for x in range(len(grid[0])):
            if xy == 'x' and x > amount:
                new_x = 2 * amount - x
            else:
                new_x = x
            if grid[y][x]:
                try:
                    new_grid[new_y][new_x] = True
                except Exception as e:
                    print(f'Indices y: {new_y}, x: {new_x} out of range')
                    raise e
    return new_grid

def create_grid_from_coords(coords):
    max_x = max(x for x, _ in coords)
    max_y = max(y for _, y in coords)
    debug_print(f'max_x: {max_x}, max_y: {max_y}')
    num_rows = max_y + 1
    num_cols = max_x + 1
    grid = create_grid(num_rows, num_cols)
    debug_print(f'num rows: {num_rows}/{len(grid)}')
    debug_print(f'num cols: {num_cols}/{len(grid[0])}')
    for x, y in coords:
        try:
            grid[y][x] = True
        except Exception as e:
            print(f'x: {x}, y: {y}')
            raise e
    return grid

def create_grid(num_rows, num_cols):
    return [[False] * num_cols for _ in range(num_rows)]

def parse_text(text):
    coords_text, commands_text = text.split('\n\n')
    return parse_coords(coords_text), parse_commands(commands_text)

def parse_coords(coords_text):
    return [parse_coord(line) for line in coords_text.split('\n')]

def parse_coord(line):
    x, y = line.split(',')
    return int(x), int(y)

def parse_commands(commands_text):
    return [parse_command(line) for line in commands_text.split('\n')]

COMMAND_PATTERN = re.compile(r'fold along (x|y)=(\d+)')
def parse_command(line):
    try:
        xy, amount = COMMAND_PATTERN.match(line).groups()
    except Exception as e:
        print(f'Could not parse line: {line}')
        raise e
    return xy, int(amount)
    
def get_text(filename):
    with open(filename) as f:
        return f.read().strip()

def print_grid(grid):
    # debug_print('\n'.join(''.join('#' if c else '.' for c in row)
    #                       for row in grid))
    debug_print()

if __name__ == '__main__':
    main()
