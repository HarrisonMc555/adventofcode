#!/usr/bin/env python3

INPUT_FILE = 'input01.txt'
# INPUT_FILE = 'example01.txt'

def main():
    lines = get_lines(INPUT_FILE)
    nums = parse_nums(lines)
    increase_count = get_increase_count(nums)
    print(increase_count)

def get_increase_count(nums):
    count = 0
    prev = nums[0]
    for num in nums[1:]:
        if num > prev:
            count += 1
        prev = num
    return count

def parse_nums(lines):
    return [int(line) for line in lines]

def get_lines(filename):
    with open(filename) as f:
        return [line.strip() for line in f.readlines() if line]

if __name__ == '__main__':
    main()
    
