#!/usr/bin/env python3

INPUT_FILE = 'input10.txt'

import unittest

def main():
    if 'INPUT_FILE' not in globals():
        unittest.main()
        return
    lines = get_lines(INPUT_FILE)
    print(score_lines(lines))

def score_lines(lines):
    return sum(score_line(line) for line in lines)

CHAR_TO_SCORE = {
    ')': 3,
    ']': 57,
    '}': 1197,
    '>': 25137,
}
def score_line(line):
    illegal_char = first_illegal_char(line)
    if illegal_char:
        return CHAR_TO_SCORE[illegal_char]
    else:
        return 0

OPEN_TO_CLOSE = {
    '(': ')',
    '[': ']',
    '{': '}',
    '<': '>',
}
def first_illegal_char(line):
    stack = []
    for c in line:
        if c in OPEN_TO_CLOSE:
            stack.append(OPEN_TO_CLOSE[c])
        else:
            expected = stack.pop()
            if c != expected:
                return c
    # raise Exception(f'No illegal chars in {line}')

def get_lines(filename):
    with open(filename) as f:
        return [line.strip() for line in f.readlines()]

TEXT_TO_EXPECTED = {
    '[({(<(())[]>[[{[]{<()<>>': None,
    '[(()[<>])]({[<{<<[]>>(': None,
    '{([(<{}[<>[]}>{[]{[(<()>': '}',
    '(((({<>}<{<{<>}{[]{[]{}': None,
    '[[<[([]))<([[{}[[()]]]': ')',
    '[{[{({}]{}}([{[{{{}}([]': ']',
    '{<[[]]>}<{[{[{[]{()[[[]': None,
    '[<(<(<(<{}))><([]([]()': ')',
    '<{([([[(<>()){}]>(<<{{': '>',
    '<{([{{}}[<[[[<>{}]]]>[]]': None,
}
EXPECTED_SCORE = 26397
class Test(unittest.TestCase):
    def test_example(self):
        for text, expected in TEXT_TO_EXPECTED.items():
            self.assertEqual(first_illegal_char(text), expected, text)
        score = score_lines(TEXT_TO_EXPECTED.keys())
        self.assertEqual(score, EXPECTED_SCORE, 'Total score')

if __name__ == '__main__':
    main()
