#!/usr/bin/env python3

INPUT_FILE = 'input14.txt'

import unittest
import re
from collections import Counter

DEBUG = False
# DEBUG = True
def debug_print(*args, **kwargs):
    if DEBUG:
        print(*args, **kwargs)

def main():
    if 'INPUT_FILE' not in globals():
        unittest.main()
        return
    text = get_text(INPUT_FILE)
    template, rules = parse_text(text)
    print(run(template, rules))

NUM_STEPS = 10
def run(template, rules):
    cur = template
    for _ in range(NUM_STEPS):
        cur = run_step(rules, cur)
    char_counter = Counter(cur)
    most_common = max(char_counter.values())
    least_common = min(char_counter.values())
    return most_common - least_common

def run_step(rules, text):
    result = []
    for c1, c2 in pairs(text):
        result.append(c1)
        result.append(rules[(c1, c2)])
    result.append(text[-1])
    return ''.join(result)

def pairs(iterable):
    it = iter(iterable)
    prev = next(it)
    for cur in it:
        yield prev, cur
        prev = cur

def parse_text(text):
    template_text, rules_text = text.split('\n\n')
    return template_text.strip(), parse_rules(rules_text)

def parse_rules(rules_text):
    return dict(parse_rule(line) for line in rules_text.strip().split('\n'))

def parse_rule(line):
    from_letters, to_letter = line.strip().split(' -> ')
    return tuple(from_letters), to_letter

def get_text(filename):
    with open(filename) as f:
        return f.read().strip()

EXAMPLE_TEXT = '''
NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C
'''
EXAMPLE_STEPS = [
    'NCNBCHB',
    'NBCCNBBBCBHCB',
    'NBBBCNCCNBBNBNBBCHBHHBCHB',
    'NBBNBNBBCCNBCNCCNBBNBBNBBBNBBNBBCBHCBHHNHCBBCBHCB',
]
STEP_TO_LENGTH = {
    5: 97,
    10: 3073,
}
class Test(unittest.TestCase):
    def test_examples(self):
        template, rules = parse_text(EXAMPLE_TEXT)
        cur = template
        num_steps = 0
        for step in EXAMPLE_STEPS:
            num_steps += 1
            cur = run_step(rules, cur)
            self.assertEqual(step, cur)
        max_steps = max(STEP_TO_LENGTH.keys())
        while num_steps < max_steps:
            num_steps += 1
            cur = run_step(rules, cur)
            if num_steps in STEP_TO_LENGTH:
                self.assertEqual(len(cur), STEP_TO_LENGTH[num_steps])
        self.assertEqual(num_steps, 10)
        char_counter = Counter(cur)
        self.assertEqual(char_counter['B'], 1749)
        self.assertEqual(char_counter['C'], 298)
        self.assertEqual(char_counter['H'], 161)
        self.assertEqual(char_counter['N'], 865)

    def test_pairs(self):
        lst = [1, 2, 3, 4]
        actual = list(pairs(lst))
        expected = [(1, 2), (2, 3), (3, 4)]
        self.assertEqual(actual, expected)

if __name__ == '__main__':
    main()
