#!/usr/bin/env python3

import itertools

# TEST = True

REAL = False
REAL = True

if REAL:
    STARTING_POSITIONS = 1, 3
else:
    STARTING_POSITIONS = 4, 8
import unittest
import re

DEBUG = False
# DEBUG = True
def debug_print(*args, **kwargs):
    if DEBUG:
        print(*args, **kwargs)

def main():
    if 'TEST' in globals():
        unittest.main()
        return
    print(run(STARTING_POSITIONS))

MAX_POINTS = 1000
def run(starting_positions):
    p1pos, p2pos = starting_positions
    p1score, p2score = 0, 0
    def status(header, turn):
        debug_print()
        debug_print(f'== {header} {turn} ==')
        debug_print(f'p1pos: {p1pos}, p1score: {p1score}')
        debug_print(f'p2pos: {p2pos}, p2score: {p2score}')
    debug_print('== Before game ==')
    debug_print(f'p1pos: {p1pos}, p1score: {p1score}')
    debug_print(f'p2pos: {p2pos}, p2score: {p2score}')
    for turn in itertools.count(1):
        status('Before turn', turn)
        num_dice_rolled = turn * 3
        if turn % 2 == 1:
            p1pos = add_pos(p1pos, turn)
            p1score += p1pos
            if p1score >= MAX_POINTS:
                debug_print(f'P1 won')
                debug_print(f'Returning {p2score} * {num_dice_rolled} = {p2score * num_dice_rolled}')
                return p2score * num_dice_rolled
        else:
            p2pos = add_pos(p2pos, turn)
            p2score += p2pos
            if p2score >= MAX_POINTS:
                debug_print('P2 won')
                debug_print(f'Returning {p1score} * {num_dice_rolled} = {p1score * num_dice_rolled}')
                return p1score * num_dice_rolled
        status('After turn', turn)

def add_pos(pos, turn):
    dice_sum = get_dice_sum(turn)
    debug_print(f'\tdice sum for turn {turn} = {dice_sum}')
    debug_print(f'\tUnwrapped pos {pos} -> {pos + dice_sum}')
    wrapped_pos = wrap_pos(pos + dice_sum)
    debug_print(f'\tWrapped pos {pos + dice_sum} -> {wrapped_pos}')
    return wrapped_pos
    # return wrap_pos(pos + get_dice_sum(turn))

def wrap_pos(pos):
    return (pos - 1) % 10 + 1

def get_dice_sum(turn):
    return turn * 9 - 3

class Test(unittest.TestCase):
    pass

if __name__ == '__main__':
    main()
