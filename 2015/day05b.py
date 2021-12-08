#!/usr/bin/env python3

INPUT_FILE = 'input05.txt'

import unittest
import re

def main():
    if 'INPUT_FILE' not in globals():
        unittest.main()
        return
    lines = get_lines(INPUT_FILE)
    print(sum(1 for line in lines if run(line)))

TWO_LETTERS_TWICE_PATTERN = re.compile(r'(..).*\1')
TWO_LETTERS_ONE_IN_BETWEEN_PATTERN = re.compile(r'(.).\1')
def run(line):
    return bool(TWO_LETTERS_TWICE_PATTERN.search(line)) and \
        bool(TWO_LETTERS_ONE_IN_BETWEEN_PATTERN.search(line))

def get_lines(filename):
    with open(filename) as f:
        return f.readlines()

TEXT_TO_EXPECTED = {
    'qjhvhtzxzqqjkmpb': True,
    'xxyxx': True,
    'uurcxstgmygtbstg': False,
    'ieodomkazucvgmuy': False,
}
class Test(unittest.TestCase):
    def test_examples(self):
        for text, expected in TEXT_TO_EXPECTED.items():
            self.assertEqual(run(text), expected, text)

if __name__ == '__main__':
    main()
