#!/usr/bin/env python3

INPUT = '3113322113'

import unittest
import re

DEBUG = False
def debug_print(*args, **kwargs):
    if DEBUG:
        print(*args, **kwargs)

NUM_TIMES = 40
def main():
    if 'INPUT' not in globals():
        unittest.main()
        return
    print(run(INPUT, NUM_TIMES))

def run(text, num_times):
    for _ in range(num_times):
        text = get_next(text)
    return len(text)

def get_next(text):
    return ''.join(str(count) + letter
                   for letter, count in get_chunks(text))

def get_chunks(iterable):
    result = []
    iterator = iter(iterable)
    cur = next(iterator)
    cur_count = 1
    for item in iterator:
        if cur == item:
            cur_count += 1
        else:
            result.append((cur, cur_count))
            cur = item
            cur_count = 1
    result.append((cur, cur_count))
    return result

TEST_SEQUENCE = [
    '1',
    '11',
    '21',
    '1211',
    '111221',
    '312211',
]
class Test(unittest.TestCase):
    def test_example(self):
        for before, after in zip(TEST_SEQUENCE, TEST_SEQUENCE[1:]):
            self.assertEqual(get_next(before), after)

    def test_chunks(self):
        self.assertEqual(get_chunks('aaabbccccd'),
                         [('a', 3), ('b', 2), ('c', 4), ('d', 1)])
        

if __name__ == '__main__':
    main()
