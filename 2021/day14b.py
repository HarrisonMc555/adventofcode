#!/usr/bin/env python3

INPUT_FILE = 'input14.txt'
# INPUT_FILE = 'example14.txt'

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

NUM_STEPS = 40
def run(template, rules):
    cur = Counter(get_pairs(template))
    for i in range(NUM_STEPS):
        cur = run_step(rules, cur)
    letter_counts = get_letter_counts(cur)
    most_common = max(letter_counts.values())
    least_common = min(letter_counts.values())
    return most_common - least_common

def run_step(rules, pairs):
    result = Counter()
    for (c1, c2), count in pairs.items():
        new_c = rules[(c1, c2)]
        result[(c1, new_c)] += count
        result[(new_c, c2)] += count
    return result

def get_letter_counts(pairs):
    double_counts = Counter()
    for (c1, c2), count in pairs.items():
        double_counts[c1] += count
        double_counts[c2] += count
    # Add one because the first/last letter will *not* be double counted.
    # Everything else will be even, and adding one before dividing by two will
    # not affect the resulting count
    letter_counts = {c: (count + 1) // 2 for c, count in double_counts.items()}
    return letter_counts

def get_pairs(iterable):
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
STEP_TO_LENGTH = {
    5: 97,
    10: 3073,
}
class Test(unittest.TestCase):
    def test_examples(self):
        template, rules = parse_text(EXAMPLE_TEXT)
        cur = Counter(get_pairs(template))
        num_steps = 0
        max_steps = max(STEP_TO_LENGTH.keys())
        while num_steps < max_steps:
            num_steps += 1
            cur = run_step(rules, cur)
            if num_steps in STEP_TO_LENGTH:
                self.assertEqual(sum(cur.values()) + 1,
                                 STEP_TO_LENGTH[num_steps])
        self.assertEqual(num_steps, 10)
        char_counter = Counter(cur)
        get_le
        self.assertEqual(char_counter['B'], 1749)
        self.assertEqual(char_counter['C'], 298)
        self.assertEqual(char_counter['H'], 161)
        self.assertEqual(char_counter['N'], 865)

    def test_pairs(self):
        lst = [1, 2, 3, 4]
        actual = list(get_pairs(lst))
        expected = [(1, 2), (2, 3), (3, 4)]
        self.assertEqual(actual, expected)

if __name__ == '__main__':
    main()
