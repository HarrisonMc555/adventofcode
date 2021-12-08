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
    row1, col1 = 0, 0
    row2, col2 = 0, 0
    seen = {(0, 0)}
    for c1, c2 in chunks(text.strip(), 2):
        drow1, dcol1 = get_movement(c1)
        row1 += drow1
        col1 += dcol1
        drow2, dcol2 = get_movement(c2)
        seen.add((row1, col1))
        row2 += drow2
        col2 += dcol2
        seen.add((row2, col2))
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

def chunks(lst, n):
    """Yield successive n-sized chunks from lst."""
    for i in range(0, len(lst), n):
        yield lst[i:i + n]

def get_text(filename):
    with open(filename) as f:
        return f.read()

TEXT_TO_EXPECTED = {
    '>v': 3,
    '^>v<': 3,
    '^v^v^v^v^v': 11,
}
class Test(unittest.TestCase):
    def test_examples(self):
        for text, expected in TEXT_TO_EXPECTED.items():
            self.assertEqual(run(text), expected)

if __name__ == '__main__':
    main()
