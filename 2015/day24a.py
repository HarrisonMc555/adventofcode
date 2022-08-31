#!/usr/bin/env python3

INPUT_FILE = 'input24.txt'

import unittest
import math

DEBUG = True
DEBUG = False
def debug_print(*args, **kwargs):
    if DEBUG:
        print(*args, **kwargs)

def main():
    if 'INPUT_FILE' not in globals():
        unittest.main()
        return
    nums = parse_lines(get_lines(INPUT_FILE))
    print(run(nums))

NUM_GROUPS = 3
def run(nums):
    nums.sort()
    debug_print(nums)
    total_sum = sum(nums)
    debug_print(f'total_sum: {total_sum}')
    if total_sum % NUM_GROUPS != 0:
        raise Exception(f'Total sum {total_sum} is not divisible by {NUM_GROUPS}')
    each_group_sum = total_sum // NUM_GROUPS
    debug_print(f'each_group_sum: {each_group_sum}')
    cur_sum = 0
    for i, num in enumerate(reversed(nums)):
        cur_sum += num
        if cur_sum >= each_group_sum:
            min_group_length = i + 1
            break
    else:
        raise Exception('Total sum wasn\'t big enough????')
    debug_print(f'min_group_length: {min_group_length}')
    max_group_length = len(nums) // NUM_GROUPS
    debug_print(f'max_group_length: {max_group_length}')
    best_product = math.inf
    found = False
    for group_length in range(min_group_length, max_group_length + 1):
        for first_group, remaining in find_groups_with_length(nums, each_group_sum, group_length):
            if group_exists(remaining, each_group_sum):
                new_product = product(first_group)
                if new_product < best_product:
                    best_product = new_product
                    found = True
        if found:
            return best_product
    raise Exception('No group found')

def group_exists(nums, target_sum):
    def helper(cur_sum, index):
        for i in range(index, len(nums)):
            num = nums[i]
            new_sum = cur_sum + num
            if new_sum > target_sum:
                continue
            if new_sum == target_sum:
                return True
            elif helper(new_sum, i + 1):
                return True
        return False
    return helper(0, 0)

def find_groups_with_length(nums, target_sum, group_length):
    skipped_nums = []
    group = []
    def helper(cur_sum, index):
        for i in range(index, len(nums)):
            num = nums[i]
            new_sum = cur_sum + num
            if new_sum > target_sum:
                continue
            group.append(num)
            if len(group) == group_length:
                if new_sum == target_sum:
                    # yield a copy
                    yield group[:], skipped_nums + nums[i + 1:]
            else:
                yield from helper(new_sum, i + 1)
            group.pop()
            skipped_nums.append(num)
    yield from helper(0, 0)

def product(nums):
    result = 1
    for num in nums:
        result *= num
    return result

def parse_lines(lines):
    return [int(line) for line in lines]

def get_lines(filename):
    with open(filename) as f:
        return [line.strip() for line in f.readlines()]

class Test(unittest.TestCase):
    def test_custom_example(self):
        nums = [1, 6, 2, 5, 3, 4]
        self.assertEqual(run(nums), 6)

    def test_find_groups_with_length(self):
        nums = list(reversed(range(1, 6 + 1)))
        actual = [group for group, _ in find_groups_with_length(nums, 7, 2)]
        expected = [[6, 1], [5, 2], [4, 3]]
        self.assertEqual(actual, expected)

        nums = list(reversed(range(1, 10 + 1)))
        actual = [group for group, _ in find_groups_with_length(nums, 13, 3)]
        expected = [
            [10, 2, 1],
            [9, 3, 1],
            [8, 4, 1],
            [8, 3, 2],
            [7, 5, 1],
            [7, 4, 2],
            [6, 5, 2],
            [6, 4, 3],
        ]
        self.assertEqual(actual, expected)

    def test_example(self):
        nums = list(range(1, 5 + 1)) + list(range(7, 11 + 1))
        self.assertEqual(run(nums), 99)

if __name__ == '__main__':
    main()
