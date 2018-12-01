#!/usr/bin/env python3

import sys

def repeat(iterable):
    lst = list(iterable)
    i = 0
    while True:
        yield lst[i]
        i = (i + 1) % len(lst)

def solve(frequencies):
    cur = 0
    seen = set()
    for freq in repeat(frequencies):
        seen.add(cur)
        cur += freq
        if cur in seen:
            return cur
    raise Exception('Impossible')

def get_input():
    return [int(w) for w in sys.stdin.readlines()]

def main():
    frequencies = get_input()
    print(solve(frequencies))

if __name__ == '__main__':
    main()
