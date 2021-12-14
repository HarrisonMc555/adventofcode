#!/usr/bin/env python3

INPUT_FILE = 'input16.txt'

import re

REAL_AUNT_SUE = '''
children: 3
cats: 7
samoyeds: 2
pomeranians: 3
akitas: 0
vizslas: 0
goldfish: 5
trees: 3
cars: 2
perfumes: 1
'''

DEBUG = True
def debug_print(*args, **kwargs):
    if DEBUG:
        print(*args, **kwargs)

def main():
    lines = get_lines(INPUT_FILE)
    all_sue_things = parse_lines(lines)
    real_sue_things = parse_real_sue_things(REAL_AUNT_SUE)
    for num, things in all_sue_things:
        if matches_real_sue(real_sue_things, things):
            print(num)
    
def matches_real_sue(real_sue_things, things):
    for thing, value in things:
        real_value = real_sue_things[thing]
        
        if thing in ['cats', 'trees']:
            matches = int(value) > int(real_value)
        elif thing in ['pomeranians', 'goldfish']:
            matches = int(value) < int(real_value)
        else:
            matches = value == real_value
        if not matches:
            return False
    return True

def parse_real_sue_things(text):
    return dict(tuple(line.split(': ')) for line in text.strip().split('\n'))

def parse_lines(lines):
    return [parse_line(line) for line in lines]

LINE_RE = re.compile(r'Sue (\d+): (.*)')
def parse_line(line):
    try:
        sue_num, things_string =  LINE_RE.match(line).groups()
        return int(sue_num), parse_things_string(things_string)
    except Exception as e:
        print(f'Could not parse line: "{line}"')
        raise e

def parse_things_string(s):
    return [tuple(w.split(': ')) for w in s.split(', ')]

def get_lines(filename):
    with open(filename) as f:
        return [line.strip() for line in f.readlines()]

if __name__ == '__main__':
    main()
