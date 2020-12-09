#!/usr/bin/env python3

from collections import Counter

INPUT_FILE = 'input09.txt'
# INPUT_FILE = 'example.txt'
PREAMBLE_LEN = 25
# PREAMBLE_LEN = 5

def main():
    lines = get_lines(INPUT_FILE)
    nums = parse_lines(lines)
    invalid_num = find_first_invalid_num(nums, PREAMBLE_LEN)
    # print(invalid_num)
    sequence = find_nums_that_sum_to(nums, invalid_num)
    weakness = min(sequence) + max(sequence)
    print(weakness)

def get_lines(filename):
    with open(filename) as f:
        return [line.strip() for line in f.readlines() if line]

def parse_lines(lines):
    return [int(line) for line in lines]

def find_first_invalid_num(nums, preamble_len):
    window = Counter(nums[:preamble_len])
    for i, num in enumerate(nums[preamble_len:]):
        # print(f'window: {window}')
        # print(f'is {num} valid?')
        if not is_valid_num(window, num):
            return num
        # print(f'{num} was valid, adding')
        window[num] += 1
        edge_num = nums[i]
        # print(f'{edge_num} was on edge, removing')
        window[edge_num] -= 1
        # print()
    raise Exception(f'No invalid numbers were found')

def is_valid_num(window, num):
    # print(f'Find pair that adds up to {num}')
    for first in window:
        if window[first] <= 0:
            continue
        second = num - first
        # print(f'\t{first} + {second} = {num}')
        if second == first:
            limit = 1
        else:
            limit = 0
        if window[second] > limit:
            # print(f'\t\t{second} is present!')
            return True
    return False

def find_nums_that_sum_to(nums, goal):
    for i, first in enumerate(nums):
        total = first
        for j in range(i + 1, len(nums)):
            other = nums[j]
            total += other
            if total == goal:
                return nums[i:j + 1]
            elif total > goal:
                break
    raise Exception(f'No sequence found that adds to {goal}')

if __name__ == '__main__':
    main()
