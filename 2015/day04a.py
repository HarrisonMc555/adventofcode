#!/usr/bin/env python3

INPUT = 'yzbqklnj'

import hashlib
import unittest

def main():
    if 'INPUT' not in globals():
        unittest.main()
        return
    print(run(INPUT))

def run(text):
    prefix = text.encode('ascii')
    for x in range(1, 1_000_000_000):
        if md5(prefix, x).startswith('00000'):
            return x

def md5(prefix, x):
    return hashlib.md5(prefix + str(x).encode('ascii')).hexdigest()

def get_text(filename):
    with open(filename) as f:
        return f.read().strip()

TEXT_TO_EXPECTED = {
    'abcdef': 609043,
    'pqrstuv': 1048970,
}
class Test(unittest.TestCase):
    def test_examples(self):
        for text, expected in TEXT_TO_EXPECTED.items():
            self.assertEqual(run(text), expected)

if __name__ == '__main__':
    main()
