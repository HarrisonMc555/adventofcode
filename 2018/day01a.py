#!/usr/bin/env python3

import sys

def solve(frequencies):
    return sum(frequencies)

def get_input():
    return [int(w) for w in sys.stdin.readlines()]

def main():
    frequencies = get_input()
    print(solve(frequencies))

if __name__ == '__main__':
    main()
