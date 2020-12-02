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
    index1, index2, letter, password = LINE_RE.match(line).groups()
    return int(index1), int(index2), letter, password

def is_valid_password(index1, index2, letter, password):
    char1_matches = password[index1 - 1] == letter
    char2_matches = password[index2 - 1] == letter
    return char1_matches ^ char2_matches

def count_matching(lst, pred):
    return len([_ for val in lst if pred(val)])

if __name__ == '__main__':
    main()
