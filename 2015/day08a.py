#!/usr/bin/env python3

INPUT_FILE = 'input08.txt'

import unittest

def main():
    if 'INPUT_FILE' not in globals():
        unittest.main()
        return
    lines = get_lines(INPUT_FILE)
    print(run(lines))

def run(lines):
    return sum(calc_line(line) for line in lines)

def calc_line(line):
    return len(line) - len(eval(line))

def get_code_len(line):
    return len(line)

def get_str_len(line):
    return len(eval(line))

def get_lines(filename):
    with open(filename) as f:
        return [line.strip() for line in f.readlines()]

LINE_CODE_LEN_STR_LEN = [
    (r'""', 2, 0),
    (r'"abc"', 5, 3),
    (r'"aaa\"aaa"', 10, 7),
    (r'"\x27"', 6, 1),
]
class Test(unittest.TestCase):
    def test_example(self):
        for line, code_len, str_len in LINE_CODE_LEN_STR_LEN:
            self.assertEqual(get_code_len(line), code_len, f'code_len: {line}')
            self.assertEqual(get_str_len(line), str_len, f'str_len: {line}')

if __name__ == '__main__':
    main()
