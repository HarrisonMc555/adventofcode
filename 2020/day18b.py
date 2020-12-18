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
    multiply_value = None
    while True:
        try:
            token = next(token_gen)
        except StopIteration:
            # print('End of tokens')
            if multiply_value is None:
                # print(f'\treturning {value}')
                return value
            else:
                # print(f'\treturning {multiply_value} * {value} = {multiply_value * value}')
                return multiply_value * value
            return value
        # print(f'token = {token}')
        if token == ')':
            if multiply_value is None:
                # print(f'\treturning {value}')
                return value
            else:
                # print(f'\treturning {multiply_value} * {value} = {multiply_value * value}')
                return multiply_value * value
        elif token == '*':
            if multiply_value is None:
                # print(f'\tstoring value of {value} as multiply_value')
                multiply_value = value
                value = eval_value(token_gen)
            else:
                # print(f'\t{value} * {multiply_value} = ', end='')
                multiply_value = multiply_value * value
                # print(multiply_value)
                value = eval_value(token_gen)
        elif token == '+':
            next_value = eval_value(token_gen)
            # print(f'\t{value} + {next_value} = ', end='')
            value = value + next_value
            # print(value)
        else:
            raise Exception(f'Invalid operator: {token}')

def eval_value(token_gen):
    token = next(token_gen)
    if token == '(':
        # print('found (, starting again')
        return eval_tokens(token_gen)
    else:
        return int(token)

class ExampleTests(unittest.TestCase):
    def test_examples(self):
        cases = [
            ('1 + (2 * 3) + (4 * (5 + 6))', 51),
            ('2 * 3 + (4 * 5)', 46),
            ('5 + (8 * 3 + 9 + 3 * 4 * 3)', 1445),
            ('5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))', 669060),
            ('((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2', 23340),
        ]
        for text, expected in cases:
            self.assertEqual(eval_math(text), expected)

if __name__ == '__main__':
    main()
