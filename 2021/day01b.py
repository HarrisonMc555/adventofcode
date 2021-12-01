#!/usr/bin/env python3

from itertools import islice

INPUT_FILE = 'input01.txt'
# INPUT_FILE = 'example01.txt'

def main():
    lines = get_lines(INPUT_FILE)
    nums = parse_nums(lines)
    increase_count = get_increase_count(nums, 3)
    print(increase_count)

def get_increase_count(nums, window_size):
    count = 0
    prev_sum = sum(nums[:window_size])
    for window in windows(nums[1:], window_size):
        cur_sum = sum(window)
        if cur_sum > prev_sum:
            count += 1
        prev_sum = cur_sum
    return count

def windows(seq, n=2):
    "Returns a sliding window (of width n) over data from the iterable"
    "   s -> (s0,s1,...s[n-1]), (s1,s2,...,sn), ...                   "
    it = iter(seq)
    result = tuple(islice(it, n))
    if len(result) == n:
        yield result
    for elem in it:
        result = result[1:] + (elem,)
        yield result

def parse_nums(lines):
    return [int(line) for line in lines]

def get_lines(filename):
    with open(filename) as f:
        return [line.strip() for line in f.readlines() if line]

if __name__ == '__main__':
    main()
