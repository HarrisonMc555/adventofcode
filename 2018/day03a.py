#!/usr/bin/env python3

import sys
import re

def solve(claims):
    return claims[0:10]

def get_input():
    return [parse_line(line) for line in sys.stdin.readlines()]

PATTERN = re.compile(r'#(\d+) @ (\d+),(\d+): (\d+)x(\d+)')
def parse_line(line):
    return tuple(PATTERN.match(line).groups())

def main():
    claims = get_input()
    print(solve(claims))

if __name__ == '__main__':
    main()
