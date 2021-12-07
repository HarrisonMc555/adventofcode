#!/usr/bin/env python3

INPUT_FILE = 'input05.txt'
# INPUT_FILE = 'example05.txt'

import re
from collections import defaultdict

def main():
    lines = parse_lines(get_lines(INPUT_FILE))
    points = get_covered_points(lines)
    # debug_print_covered_points(lines, points)
    print(sum(1 for count in points.values() if count >= 2))

def count_gen(gen):
    return sum(1 for _ in gen)

def get_covered_points(lines):
    covered_points = defaultdict(int)
    for line in lines:
        for point in get_covered_points_from_line(line):
            covered_points[point] += 1
    return covered_points

def get_covered_points_from_line(line):
    x1, y1, x2, y2 = line
    if x1 == x2:
        return [(x1, y) for y in get_range(y1, y2)]
    elif y1 == y2:
        return [(x, y1) for x in get_range(x1, x2)]
    else:
        return []

def get_range(num1, num2):
    small = min(num1, num2)
    big = max(num1, num2)
    return range(small, big + 1)

def parse_lines(text_lines):
    return [parse_line(text_line) for text_line in text_lines]

LINE_PATTERN = re.compile('(\d+),(\d+) -> (\d+),(\d+)')
def parse_line(text_line):
    num_strings = LINE_PATTERN.match(text_line).groups()
    return [int(s) for s in num_strings]

def get_lines(filename):
    with open(filename) as f:
        return [line.strip() for line in f.readlines() if line]

def debug_print_covered_points(lines, points):
    xs = [x1 for x1, _, _, _ in lines] + [x2 for _, _, x2, _ in lines]
    ys = [y1 for _, y1, _, _ in lines] + [y2 for _, _, _, y2 in lines]
    xmin, xmax = min(xs), max(xs)
    ymin, ymax = min(ys), max(ys)
    for y in range(ymin, ymax + 1):
        for x in range(xmin, xmax + 1):
            point = (x, y)
            if point in points:
                print(points[point], end='')
            else:
                print('.', end='')
        print()
    print()

if __name__ == '__main__':
    main()
