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

DISALLOWED = ['ab', 'cd', 'pq', 'xy']
def run(line):
    return count_vowels(line) >= 3 and \
        contains_double_letter(line) and \
        not contains_any(line, DISALLOWED)

VOWELS = set('aeiou')
def count_vowels(text):
    return sum(1 for c in text if c in VOWELS)

DOUBLE_LETTER_PATTERN = re.compile(r'(.)\1')
def contains_double_letter(text):
    return bool(DOUBLE_LETTER_PATTERN.search(text))

def contains_any(text, disallowed):
    return any(p in text for p in disallowed)

def get_lines(filename):
    with open(filename) as f:
        return f.readlines()

TEXT_TO_EXPECTED = {
    'ugknbfddgicrmopn': True,
    'aaa': True,
    'jchzalrnumimnmhp': False,
    'haegwjzuvuyypxyu': False,
    'dvszwmarrgswjxmb': False,
}
class Test(unittest.TestCase):
    def test_examples(self):
        for text, expected in TEXT_TO_EXPECTED.items():
            self.assertEqual(run(text), expected, text)

if __name__ == '__main__':
    main()
