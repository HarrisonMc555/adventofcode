#!/usr/bin/env python3

from itertools import islice

INPUT_FILE = 'input01.txt'
# INPUT_FILE = 'example01.txt'

def main():
    lines = get_lines(INPUT_FILE)
    nums = parse_nums(lines)
    increase_count = get_increase_count(sums_gen(nums, 3))
    print(increase_count)

def sums_gen(nums, window_size):
    prev_sum = sum(nums[:window_size])
    yield prev_sum
    for i in range(len(nums) - window_size):
        cur_sum = prev_sum - nums[i] + nums[i + window_size]
        yield cur_sum

def get_increase_count(nums_gen):
    it = iter(nums_gen)
    prev = next(it)
    count = 0
    for cur in it:
        if cur > prev:
            count += 1
    return count

def parse_nums(lines):
    return [int(line) for line in lines]

def get_lines(filename):
    with open(filename) as f:
        return [line.strip() for line in f.readlines() if line]

if __name__ == '__main__':
    main()
