#!/usr/bin/env python3

INPUT_FILE = 'input17.txt'
# INPUT_FILE = 'example17.txt'

import unittest
import math

DEBUG = True
def debug_print(*args, **kwargs):
    if DEBUG:
        print(*args, **kwargs)

TOTAL = 150
# TOTAL = 25
def main():
    if 'INPUT_FILE' not in globals():
        unittest.main()
        return
    nums = parse_lines(get_lines(INPUT_FILE))
    least = find_least(nums, TOTAL)
    print(num_combinations_for_total_with_length(nums, TOTAL, least))
    
def find_least(nums, total):
    nums.sort(reverse=True)
    return find_least_helper(nums, total, 0)

def find_least_helper(nums, cur, length):
    if cur < 0:
        return math.inf
    if cur == 0:
        return length
    if not nums:
        return math.inf
    num = nums[0]
    rest = nums[1:]
    return min(find_least_helper(rest, cur, length),
               find_least_helper(rest, cur - num, length + 1))

def num_combinations_for_total_with_length(nums, total, length):
    nums.sort(reverse=True)
    return helper(nums, total, length)

def helper(nums, cur, length):
    if cur < 0:
        return 0
    if cur == 0:
        if length == 0:
            return 1
        else:
            return 0
    if not nums:
        return 0
    num = nums[0]
    rest = nums[1:]
    return helper(rest, cur, length) + helper(rest, cur - num, length - 1)

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
