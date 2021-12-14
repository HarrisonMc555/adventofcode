#!/usr/bin/env python3

INPUT_FILE = 'input17.txt'

import unittest

DEBUG = True
def debug_print(*args, **kwargs):
    if DEBUG:
        print(*args, **kwargs)

TOTAL = 150
def main():
    if 'INPUT_FILE' not in globals():
        unittest.main()
        return
    nums = parse_lines(get_lines(INPUT_FILE))
    print(num_combinations_for_total(nums, TOTAL))
    
def num_combinations_for_total(nums, total):
    nums.sort(reverse=True)
    return helper(nums, total)

def helper(nums, cur):
    if cur < 0:
        return 0
    if cur == 0:
        return 1
    if not nums:
        return 0
    num = nums[0]
    rest = nums[1:]
    return helper(rest, cur) + helper(rest, cur - num)

def parse_lines(lines):
    return [int(line) for line in lines]

def get_lines(filename):
    with open(filename) as f:
        return [line.strip() for line in f.readlines()]

NUMS_GOAL_COMBINATIONS = [
    ([1], 1, 1),
    ([1], 2, 0),
    ([1], 3, 0),
    ([1, 2], 3, 1),
    ([1, 2], 4, 0),
    ([1, 2, 3], 4, 1),
    ([1, 2, 3, 2], 4, 2),
    ([1, 2, 3, 2], 3, 3),
]
class Test(unittest.TestCase):
    def test_examples(self):
        for nums, goal, combinations in NUMS_GOAL_COMBINATIONS:
            self.assertEqual(num_combinations_for_total(nums, goal),
                             combinations)

if __name__ == '__main__':
    main()
