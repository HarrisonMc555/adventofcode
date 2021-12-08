#!/usr/bin/env python3

TEST = True
TEST = False
INPUT_FILE = 'input02.txt'

import re

def main():
    if TEST:
        test()
        return
    else:
        print(run(get_text(INPUT_FILE).strip()))

def run(text):
    all_dimensions = parse_text(text)
    return sum(get_ribbon(dim) for dim in all_dimensions)

def get_ribbon(dimensions):
    l, w, h = dimensions
    lw = l + w
    lh = l + h
    wh = w + h
    smallest = min(lw, min(lh, wh))
    volume = l * w * h
    return 2 * smallest + volume
    
def parse_text(text):
    return [parse_line(line) for line in text.strip().split('\n')]

LINE_RE = re.compile('(\d+)x(\d+)x(\d+)')
def parse_line(line):
    return [int(w) for w in LINE_RE.match(line).groups()]

def get_text(filename):
    with open(filename) as f:
        return f.read().strip()

def test():
    test_cases = {
        '2x3x4': 34,
        '1x1x10': 14,
    }
    success = True
    for text, expected in test_cases.items():
        actual = run(text)
        if actual != expected:
            success = False
            print(f'FAILED: got {actual}, expected {expected} from {text}')
    if success:
        print('SUCCESS')

if __name__ == '__main__':
    main()
