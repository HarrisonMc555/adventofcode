#!/usr/bin/env python3

INPUT_FILE = 'input19.txt'

import unittest
import re

DEBUG = True
def debug_print(*args, **kwargs):
    if DEBUG:
        print(*args, **kwargs)

def main():
    if 'INPUT_FILE' not in globals():
        unittest.main()
        return
    rules, start = parse_text(get_text(INPUT_FILE))
    print(len(get_possibilities(rules, start)))

def get_possibilities(rules, text):
    possibilities = set()
    for rule in rules:
        possibilities.update(run_rule(rule, text))
    return possibilities

def run_rule(rule, text):
    before, after = rule
    pattern = re.compile(before)
    for match in pattern.finditer(text):
        i1, i2 = match.span()
        yield text[:i1] + after + text[i2:]

def parse_text(text):
    rules_text, start = [piece.strip() for piece in text.split('\n\n')]
    return parse_rules(rules_text), start

def parse_rules(text):
    return [parse_rule(line.strip()) for line in text.split('\n')]

def parse_rule(line):
    return tuple(line.split(' => '))

def get_text(filename):
    with open(filename) as f:
        return f.read().strip()

EXAMPLE_TEXT = '''
H => HO
H => OH
O => HH

HOH
'''
EXAMPLE_POSSIBILITIES = {
    'HOOH',
    'HOHO',
    'OHOH',
    'HOOH',
    'HHHH',
}
class Test(unittest.TestCase):
    def test_examples(self):
        rules, start = parse_text(EXAMPLE_TEXT)
        self.assertEqual(get_possibilities(rules, start),
                         EXAMPLE_POSSIBILITIES)

if __name__ == '__main__':
    main()
