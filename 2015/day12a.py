#!/usr/bin/env python3

INPUT_FILE = 'input12.txt'

import unittest
import json

DEBUG = False
def debug_print(*args, **kwargs):
    if DEBUG:
        print(*args, **kwargs)

def main():
    if 'INPUT_FILE' not in globals():
        unittest.main()
        return
    text = get_text(INPUT_FILE)
    print(get_sum(text))

def get_sum(text):
    decoded = json.loads(text)
    return sum(get_nums(decoded))

def get_nums(obj):
    if type(obj) is list:
        for sub_obj in obj:
            yield from get_nums(sub_obj)
    elif type(obj) is dict:
        for sub_obj in obj.values():
            yield from get_nums(sub_obj)
    elif type(obj) is int:
        yield obj
    else:
        pass

def get_text(filename):
    with open(filename) as f:
        return f.read().strip()

TEXT_TO_SUM = [
    ('[1,2,3]', 6),
    ('{"a":2,"b":4}', 6),
    ('[[[3]]]', 3),
    ('{"a":{"b":4},"c":-1}', 3),
    ('[-1,{"a":1}]', 0),
    ('[-1,{"a":1}]', 0),
    ('[]', 0),
    ('{}', 0),
]
class Test(unittest.TestCase):
    def test_examples(self):
        for text, json_sum in TEXT_TO_SUM:
            self.assertEqual(get_sum(text), json_sum, text)

if __name__ == '__main__':
    main()
