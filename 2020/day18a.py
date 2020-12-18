#!/usr/bin/env python3

import re
import unittest

INPUT_FILE = 'input18.txt'

def main():
    # unittest.main()
    text = get_text(INPUT_FILE)
    lines = text.split('\n')
    values = [eval_math(line) for line in lines]
    print(sum(values))

def get_text(filename):
    with open(filename) as f:
        return f.read().strip()

def parse_text(text):
    return text.split('\n')

# TOKEN_RE = re.compile(r'(\d+|\+|\*\(|\))')
TOKEN_RE = re.compile(r'(\d+|[^ ])')
def eval_math(line):
    tokens = TOKEN_RE.findall(line)
    # print(f'tokens = {tokens}')
    return eval_tokens(t for t in tokens)

def eval_tokens(token_gen):
    value = eval_value(token_gen)
    # print(f'value = {value}')
    while True:
        try:
            token = next(token_gen)
        except StopIteration:
            # print('End of tokens')
            return value
        # print(f'token = {token}')
        if token == ')':
            return value
        op = parse_op(token)
        next_value = eval_value(token_gen)
        # print(f'next_value = {next_value}')
        # print(f'{value} {token} {next_value} = ', end='')
        value = op(value, next_value)
        # print(value)

def eval_value(token_gen):
    token = next(token_gen)
    if token == '(':
        return eval_tokens(token_gen)
    else:
        return int(token)

def parse_op(token):
    if token == '+':
        return lambda x, y: x + y
    elif token == '*':
        return lambda x, y: x * y
    else:
        raise Exception(f'Invalid op token: {token}')

class ExampleTests(unittest.TestCase):
    def test_examples(self):
        cases = [
            ('1 + (2 * 3) + (4 * (5 + 6))', 51),
            ('2 * 3 + (4 * 5)', 26),
            ('5 + (8 * 3 + 9 + 3 * 4 * 3)', 437),
            ('5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))', 12240),
            ('((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2', 13632),
        ]
        for text, expected in cases:
            self.assertEqual(eval_math(text), expected)

if __name__ == '__main__':
    main()
