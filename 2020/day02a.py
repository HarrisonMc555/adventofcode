#!/usr/bin/env python3

import re

INPUT_FILE = 'input02.txt'

def main():
    lines = get_lines_from_file(INPUT_FILE)
    num_valid_passwords = [is_valid_password(*tup) for tup in lines].count(True)
    print(num_valid_passwords)

def get_lines_from_file(filename):
    with open(filename) as f:
        return parse_lines(f.readlines())

def parse_lines(lines):
    return [parse_line(line) for line in lines if line]

LINE_RE = re.compile('(\d+)-(\d+) ([a-z]): ([a-z]+)')
def parse_line(line):
    count_min, count_max, letter, password = LINE_RE.match(line).groups()
    return int(count_min), int(count_max), letter, password

def is_valid_password(count_min, count_max, letter, password):
    num_letters = password.count(letter)
    return count_min <= num_letters <= count_max

def count_matching(lst, pred):
    return len([_ for val in lst if pred(val)])

if __name__ == '__main__':
    main()
