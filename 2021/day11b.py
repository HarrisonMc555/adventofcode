#!/usr/bin/env python3

INPUT_FILE = 'input11.txt'

import unittest

MAX_CELL = 9
NUM_STEPS = 100
DEBUG = False

def main():
    if 'INPUT_FILE' not in globals():
        unittest.main()
        return
    lines = get_lines(INPUT_FILE)
    board = parse_lines(lines)
    print(get_first_sync(board))

def get_first_sync(board):
    num_cells = len(board) * len(board[0])
    num_step = 0
    while True:
        num_step += 1
        num_flashes = step(board)
        if num_flashes == num_cells:
            return num_step

def get_total_flashes(board, num_steps):
    total_flashes = 0
    for _ in range(num_steps):
        num_flashes = step(board)
        total_flashes += num_flashes
    return total_flashes

def step(board):
    will_flash = set()
    flashed = set()
    for row, col in get_indices(board):
        board[row][col] += 1
    print_board(board, 'After incrementing all by one:')
    for i in get_indices(board):
        row, col = i
        cell = board[row][col]
        if cell > MAX_CELL:
            will_flash.add(i)
    debug_print(f'Will flash: {will_flash}')
    while will_flash:
        i = will_flash.pop()
        debug_print(f'Flashing: {i}')
        debug_print(f'Flashed: {flashed}')
        debug_print(f'Will flash: {will_flash}')
        for neighbor_i in get_neighbor_indices(board, i):
            debug_print(f'\tIncrementing neighbor: {neighbor_i}')
            nrow, ncol = neighbor_i
            debug_print(f'\tNeighbor went from {board[nrow][ncol]}', end='')
            board[nrow][ncol] += 1
            debug_print(f' to {board[nrow][ncol]}')
            if board[nrow][ncol] > MAX_CELL and \
               neighbor_i not in will_flash and \
               neighbor_i not in flashed:
                debug_print(f'\t\tAdding {neighbor_i} to will flash')
                will_flash.add(neighbor_i)
        flashed.add(i)
    for row, col in get_indices(board):
        if board[row][col] > MAX_CELL:
            board[row][col] = 0
    return len(flashed)

NEIGHBOR_DIFFS = [
    (-1, -1),
    (-1, +0),
    (-1, +1),
    (+0, -1),
    # (+0, +0),
    (+0, +1),
    (+1, -1),
    (+1, +0),
    (+1, +1),
]
def get_neighbor_indices(board, index):
    row, col = index
    neighbor_indices = [(row + dr, col + dc) for dr, dc in NEIGHBOR_DIFFS]
    return [i for i in neighbor_indices if in_range(board, i)]

def in_range(board, index):
    row, col = index
    return row in range(len(board)) and col in range(len(board[0]))

def get_indices(board):
    return [(r, c) for r in range(len(board)) for c in range(len(board[0]))]

def parse_lines(lines):
    return [[int(c) for c in line] for line in lines]

def format_board(board):
    return [''.join(str(x) for x in row) for row in board]

def print_board(board, header=None):
    if type(board) != list or type(board[0]) != list:
        raise Exception(f'Invalid board argument: {board}')
    if header:
        debug_print(header)
    debug_print('\n'.join(''.join(str(x) for x in row) for row in board))
    debug_print()

def debug_print(*args, **kwargs):
    if DEBUG:
        print(*args, **kwargs)

def get_lines(filename):
    with open(filename) as f:
        return [line.strip() for line in f.readlines()]

SMALL_EXAMPLE_STEPS = [
    [
        '11111',
        '19991',
        '19191',
        '19991',
        '11111',
    ],
    [
        '34543',
        '40004',
        '50005',
        '40004',
        '34543',
    ],
    [
        '45654',
        '51115',
        '61116',
        '51115',
        '45654',
    ]
]
MEDIUM_EXAMPLE_STARTING_LINES = [
    '5483143223',
    '2745854711',
    '5264556173',
    '6141336146',
    '6357385478',
    '4167524645',
    '2176841721',
    '6882881134',
    '4846848554',
    '5283751526',
]
MEDIUM_NUM_STEPS_TO_NUM_FLASHES = [
    (10, 204),
    (10, 1656),
]
class Test(unittest.TestCase):
    def test_small(self):
        starting_lines = SMALL_EXAMPLE_STEPS[0]
        board = parse_lines(starting_lines)
        print_board(board, 'Starting board:')
        for step_lines in SMALL_EXAMPLE_STEPS[1:]:
            num_flashes = step(board)
            print_board(board, 'After step:')
            self.assertEqual(format_board(board), step_lines)

    def test_medium(self):
        starting_lines = MEDIUM_EXAMPLE_STARTING_LINES
        for num_steps, num_flashes in MEDIUM_NUM_STEPS_TO_NUM_FLASHES:
            board = parse_lines(starting_lines)
            total_flashes = get_total_flashes(board, num_steps)

    def test_medium_sync(self):
        starting_lines = MEDIUM_EXAMPLE_STARTING_LINES
        board = parse_lines(starting_lines)
        first_sync = get_first_sync(board)
        self.assertEqual(first_sync, 195)

if __name__ == '__main__':
    main()
