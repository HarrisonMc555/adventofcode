#!/usr/bin/env python3

import sys
from collections import Counter

def has_n_of_letter(num, box_id):
    counts = Counter(box_id)
    return num in counts.values()

def has_two_of_letter(box_id):
    return has_n_of_letter(2, box_id)

def has_three_of_letter(box_id):
    return has_n_of_letter(3, box_id)

def count(fun, enumerable):
    return sum(1 for val in enumerable if fun(val))

def solve(box_ids):
    two_count = count(has_two_of_letter, box_ids)
    three_count = count(has_three_of_letter, box_ids)
    return two_count * three_count

def get_input():
    return sys.stdin.readlines()

def main():
    box_ids = get_input()
    print(solve(box_ids))

if __name__ == '__main__':
    main()
