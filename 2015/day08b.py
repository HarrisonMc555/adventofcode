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
    return get_encoded_len(line) - get_code_len(line)

def get_code_len(line):
    return len(line)

def get_encoded_len(line):
    return len(get_encoded(line))

def get_encoded(line):
    return '"' + line.replace('\\', '\\\\').replace(r'"', r'\"') + '"'

def get_lines(filename):
    with open(filename) as f:
        return [line.strip() for line in f.readlines()]

LINE_CODE_LEN_ENCODED_STR_ENCODED_LEN = [
    (r'""', 2, r'"\"\""', 6),
    (r'"abc"', 5, r'"\"abc\""', 9),
    (r'"aaa\"aaa"', 10, r'"\"aaa\\\"aaa\""', 16),
    (r'"\x27"', 6, r'"\"\\x27\""', 11),
]
class Test(unittest.TestCase):
    def test_example(self):
        for line, code_len, encoded_str, encoded_len in \
            LINE_CODE_LEN_ENCODED_STR_ENCODED_LEN:
            self.assertEqual(get_code_len(line), code_len, f'code_len: {line}')
            self.assertEqual(get_encoded(line), encoded_str, f'encoded_str: {line}')
            self.assertEqual(get_encoded_len(line), encoded_len, f'encoded_len: {line}')

if __name__ == '__main__':
    main()
