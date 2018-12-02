#!/usr/bin/env python3

import sys
from collections import Counter

def remove_char(string, index):
    return ''.join(c for i, c in enumerate(string) if i != index)

def find_common_letters(box_ids, index):
    remaining_letters = Counter(remove_char(box_id, index)
                                for box_id in box_ids)
    matching = [letters for letters, count in remaining_letters.items()
                if count > 1]
    if matching:
        return matching[0]
    return None

def solve(box_ids):
    length = len(box_ids[0])
    for i in range(length):
        matching = find_common_letters(box_ids, i)
        if matching:
            return matching
    raise Exception('No common letters found')

def get_input():
    return [line.strip() for line in sys.stdin.readlines()]

def main():
    box_ids = get_input()
    print(solve(box_ids))

if __name__ == '__main__':
    main()
