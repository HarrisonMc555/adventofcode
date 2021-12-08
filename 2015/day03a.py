#!/usr/bin/env python3

INPUT_FILE = 'input03.txt'

import unittest

def main():
    if 'INPUT_FILE' not in globals():
        unittest.main()
        return
    text = get_text(INPUT_FILE)
    print(run(text))

def run(text):
    row, col = 0, 0
    seen = {(0, 0)}
    for c in text.strip():
        drow, dcol = get_movement(c)
        row += drow
        col += dcol
        seen.add((row, col))
    return len(seen)

def get_movement(c):
    if c == '^':
        return -1, 0
    elif c == '>':
        return 0, 1
    elif c == 'v':
        return 1, 0
    elif c == '<':
        return 0, -1
    else:
        raise Exception(f'Unexpected command "{c}"')

def get_text(filename):
    with open(filename) as f:
        return f.read()

TEXT_TO_EXPECTED = {
    '>v': 2,
    '^>v<': 4,
    '^v^v^v^v^v': 2,
}
class Test(unittest.TestCase):
    def test_examples(self):
        for text, expected in TEXT_TO_EXPECTED.items():
            self.assertEqual(run(text), expected)

if __name__ == '__main__':
    main()
