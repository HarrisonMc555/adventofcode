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
    scores = sorted([score for line in lines if (score := score_line(line)) > 0])
    winning_index = len(scores) // 2
    return scores[winning_index]

def score_line(line):
    completion = get_completion(line)
    if completion:
        return get_score(completion)
    else:
        return 0

OPEN_TO_CLOSE = {
    '(': ')',
    '[': ']',
    '{': '}',
    '<': '>',
}
def get_completion(line):
    stack = []
    for c in line:
        if c in OPEN_TO_CLOSE:
            stack.append(OPEN_TO_CLOSE[c])
        else:
            expected = stack.pop()
            if c != expected:
                return None
    return ''.join(reversed(stack))

CHAR_TO_SCORE = {
    ')': 1,
    ']': 2,
    '}': 3,
    '>': 4,
}
def get_score(completion):
    score = 0
    for c in completion:
        score *= 5
        score += CHAR_TO_SCORE[c]
    return score

def get_lines(filename):
    with open(filename) as f:
        return [line.strip() for line in f.readlines()]

TEXT_TO_EXPECTED = {
    '[({(<(())[]>[[{[]{<()<>>': '}}]])})]',
    '[(()[<>])]({[<{<<[]>>(': ')}>]})',
    '{([(<{}[<>[]}>{[]{[(<()>': None,
    '(((({<>}<{<{<>}{[]{[]{}': '}}>}>))))',
    '[[<[([]))<([[{}[[()]]]': None,
    '[{[{({}]{}}([{[{{{}}([]': None,
    '{<[[]]>}<{[{[{[]{()[[[]': ']]}}]}]}>',
    '[<(<(<(<{}))><([]([]()': None,
    '<{([([[(<>()){}]>(<<{{': None,
    '<{([{{}}[<[[[<>{}]]]>[]]': '])}>',
}
COMPLETION_TO_SCORE = {
    '}}]])})]': 288957,
    ')}>]})': 5566,
    '}}>}>))))': 1480781,
    ']]}}]}]}>': 995444,
    '])}>': 294,
}
EXPECTED_SCORE = 288957
class Test(unittest.TestCase):
    def test_example(self):
        for text, expected in TEXT_TO_EXPECTED.items():
            self.assertEqual(get_completion(text), expected, text)
        for completion, score in COMPLETION_TO_SCORE.items():
            self.assertEqual(get_score(completion), score, completion)
        score = score_lines(TEXT_TO_EXPECTED.keys())
        self.assertEqual(score, EXPECTED_SCORE, 'Total score')

if __name__ == '__main__':
    main()
