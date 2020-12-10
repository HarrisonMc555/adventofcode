#!/usr/bin/env python3

from collections import Counter

INPUT_FILE = 'input10.txt'
# INPUT_FILE = 'example.txt'

def main():
    lines = get_lines(INPUT_FILE)
    adapters = parse_lines(lines)
    adapters.sort()
    print(find_combinations(adapters))

def get_lines(filename):
    with open(filename) as f:
        return [line.strip() for line in f.readlines() if line]

def parse_lines(lines):
    return [int(line) for line in lines]

def find_combinations(adapters):
    paths = {a: 0 for a in adapters}
    paths[0] = 1
    max_adapter = max(adapters)
    paths[max_adapter] = 0

    add_to_paths(paths, 0)

    for adapter in adapters:
        add_to_paths(paths, adapter)

    return paths[max_adapter]
    
def add_to_paths(paths, adapter):
    count = paths[adapter]
    for diff in [1, 2, 3]:
        new_count = adapter + diff
        if new_count not in paths:
            continue
        paths[new_count] += count

if __name__ == '__main__':
    main()
