#!/usr/bin/env python3

INPUT = 36000000

import unittest

DEBUG = True
def debug_print(*args, **kwargs):
    if DEBUG:
        print(*args, **kwargs)

MAX_NUM = 1_000_000
def main():
    print(lowest_over_limit(INPUT, MAX_NUM))

def lowest_over_limit(limit, max_num):
    counts = [0] * (max_num + 1)
    for elf in range(1, max_num):
        index = elf
        addend = elf * 11
        for _ in range(50):
            if index >= len(counts):
                break
            counts[index] += addend
            index += elf
    for index, count in enumerate(counts):
        if count >= limit:
            return index
    else:
        raise Exception(f'None found, largest: {max(counts)}')

if __name__ == '__main__':
    main()
