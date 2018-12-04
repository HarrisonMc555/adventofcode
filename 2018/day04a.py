#!/usr/bin/env python3

import sys
import re

def solve(records):
    return records

def get_input():
    return [parse_record(line.strip()) for line in sys.stdin.readlines()]

PATTERN = re.compile(r'\[(\d{4})-(\d{2})-(\d{2}) (\d{2}):(\d{2})\] (.*)')
def parse_record(line):
    return PATTERN.match(line)

def main():
    records = get_input()
    # print(solve(records))
    print([record.groups() for record in records[:10]])

if __name__ == '__main__':
    main()
