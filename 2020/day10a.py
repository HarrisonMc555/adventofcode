#!/usr/bin/env python3

from collections import Counter

INPUT_FILE = 'input10.txt'
# INPUT_FILE = 'example.txt'

def main():
    lines = get_lines(INPUT_FILE)
    adapters = parse_lines(lines)
    ordered = find_order(adapters)
    differences = get_differences(ordered)
    num_ones = differences.count(1)
    num_threes = differences.count(3) + 1
    print(num_ones * num_threes)

def get_lines(filename):
    with open(filename) as f:
        return [line.strip() for line in f.readlines() if line]

def parse_lines(lines):
    return [int(line) for line in lines]

def find_order(adapters):
    adapters = list(sorted(adapters))
    prev_joltage = 0
    for joltage in adapters:
        if joltage - prev_joltage not in {1, 2, 3}:
            raise Exception(f'Invalid sequence of adapters at {joltage}')
        prev_joltage = joltage
    return adapters

def get_differences(adapters):
    differences = []
    prev_joltage = 0
    for joltage in adapters:
        differences.append(joltage - prev_joltage)
        prev_joltage = joltage
    return differences

if __name__ == '__main__':
    main()
