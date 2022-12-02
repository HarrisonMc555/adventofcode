#!/usr/bin/env python3

import unittest
import re
import itertools

TEST = True

# INPUT_FILE = 'example22a.txt'
# INPUT_FILE = 'example22b.txt'
INPUT_FILE = 'input22.txt'
MIN_VAL = -50
MAX_VAL = 50

DEBUG = True
def debug_print(*args, **kwargs):
    if 'DEBUG' in globals():
        print(*args, **kwargs)

def main():
    if 'TEST' in globals():
        unittest.main()
        return
    lines = get_lines(INPUT_FILE)
    commands = parse_lines(lines)
    state = run_commands(commands)
    # print(max(state))
    # print(min(state))
    print(len(state))

def run_commands(commands):
    state = set()
    for command in commands:
        state = run_command(state, command)
    return state

def run_command(state, command):
    on, nums = command
    x1, x2, y1, y2, z1, z2 = nums
    x1 = max(MIN_VAL, x1)
    x2 = min(MAX_VAL, x2)
    y1 = max(MIN_VAL, y1)
    y2 = min(MAX_VAL, y2)
    z1 = max(MIN_VAL, z1)
    z2 = min(MAX_VAL, z2)
    for x in range(x1, x2 + 1):
        for y in range(y1, y2 + 1):
            for z in range(z1, z2 + 1):
                cuboid = (x, y, z)
                if on:
                    state.add(cuboid)
                else:
                    state.discard(cuboid)

def parse_lines(lines):
    return [parse_line(line) for line in lines]

LINE_RE = re.compile(r'(on|off) x=(-?\d+)\.\.(-?\d+),y=(-?\d+)\.\.(-?\d+),z=(-?\d+)\.\.(-?\d+)')
def parse_line(line):
    match = LINE_RE.match(line)
    strings = list(match.groups())
    on_off = strings.pop(0)
    on = on_off == 'on'
    nums = [int(s) for s in strings]
    c1 = (nums[0], nums[2], nums[4])
    c2 = (nums[1], nums[3], nums[5])
    return on, Rect(c1, c2)

class Rect:
    def __init__(self, corner1, corner2):
        if len(corner1) != len(corner2):
            raise Exception(f'Corners do not have the same number of ' + \
                            f'dimensions: {len(corner1)} vs. {len(corner2)}')
        corner_min = min(corner1, corner2)
        corner_max = max(corner1, corner2)
        self.corner_min = corner_min
        self.corner_max = corner_max
        if len(corner_min) != len(corner_max):
            raise Exception(f'Corners do not have the same number of ' + \
                            f'dimensions: {len(corner_min)} vs. {len(corner_max)}')
        self.num_dimensions = len(corner_min)

    def __repr__(self):
        return f'Rect({self.corner_min}, {self.corner_max})'

    def __str__(self):
        return repr(self)

    def __eq__(self, other):
        if not isinstance(other, Rect):
            return False
        return self.corner_min == other.corner_min and \
            self.corner_max == other.corner_max

    def __hash__(self):
        return hash((self.corner_min, self.corner_max))

    def corners(self):
        pairs = [[self.corner_min[di], self.corner_max[di]] for di in range(self.num_dimensions)]
        return itertools.product(*pairs)

    def additional_corners(self):
        corners = list(self.corners())
        del corners[0]
        del corners[-1]
        return corners

    def contains(self, point):
        if len(point) != self.num_dimensions:
            msg = f'This rect has {self.num_dimensions} dimensions, point has ' + \
                f'{len(point)} dimenseions.'
            raise Exception(msg)
        for di in range(self.num_dimensions):
            c1 = self.corner_min[di]
            c2 = self.corner_max[di]
            p = point[di]
            if not (c1 <= p <= c2):
                return False
        else:
            return True

    def intersects(self, other):
        return any(self.contains(c) for c in other.corners()) or \
            any(other.contains(c) for c in self.corners())

    def remove_overlap_with(self, other):
        rect_ac = Rect(self.corner_min, other.corner_min)
        rect_cb = Rect(other.corner_min, self.corner_max)
        corners_ac = rect_ac.additional_corners()
        corners_cb = rect_cb.additional_corners()
        yield rect_ac
        yield from (Rect(ac, cb) for ac, cb in zip(corners_ac, corners_cb))

class Test(unittest.TestCase):
    def tests_parse_line(self):
        command = parse_line('on x=10..12,y=10..12,z=10..12')
        on, rect = command
        self.assertTrue(on)
        self.assertEqual((10, 10, 10), rect.corner_min)
        self.assertEqual((12, 12, 12), rect.corner_max)

    def test_contains(self):
        c1 = (12, 14, 16)
        c2 = (22, 24, 26)
        r = Rect(c1, c2)
        self.assertTrue(r.contains((17, 17, 17)))
        self.assertTrue(r.contains((12, 14, 16)))
        self.assertTrue(r.contains((22, 24, 26)))
        self.assertFalse(r.contains((22, 24, 27)))
        self.assertFalse(r.contains((22, 25, 26)))
        self.assertFalse(r.contains((23, 24, 26)))
        self.assertFalse(r.contains((12, 12, 12)))
        self.assertFalse(r.contains((26, 26, 26)))

    def test_corners(self):
        c1 = (5,)
        c2 = (7,)
        r = Rect(c1, c2)
        self.assertEqual({(5,), (7,)}, set(r.corners()))

        c1 = (5, 6)
        c2 = (7, 8)
        r = Rect(c1, c2)
        self.assertEqual({(5, 6), (5, 8), (7, 6), (7, 8)}, set(r.corners()))

        c1 = (10, 11, 12)
        c2 = (20, 21, 22)
        r = Rect(c1, c2)
        corners = {(10, 11, 12), (10, 11, 22), (10, 21, 12), (10, 21, 22), 
                   (20, 11, 12), (20, 11, 22), (20, 21, 12), (20, 21, 22)}
        self.assertEqual(corners, set(r.corners()))

    def test_intersects(self):
        r1 = Rect((0, 0), (10, 10))
        r2 = Rect((5, 5), (15, 15))
        self.assertTrue(r1.intersects(r2))
        self.assertTrue(r2.intersects(r1))
        r3 = Rect((100, 100), (100, 100))
        self.assertFalse(r1.intersects(r3))
        self.assertFalse(r3.intersects(r1))
        self.assertFalse(r2.intersects(r3))
        self.assertFalse(r3.intersects(r2))
        r4 = Rect((0, 20), (10, 30))
        self.assertFalse(r1.intersects(r4))
        self.assertFalse(r4.intersects(r1))
        self.assertFalse(r2.intersects(r4))
        self.assertFalse(r4.intersects(r2))
        r5 = Rect((99, 99), (101, 101))
        self.assertTrue(r3.intersects(r5))
        self.assertTrue(r5.intersects(r3))

    def test_remove_overlap_with(self):
        r1 = Rect((10, 10), (20, 20))
        r2 = Rect((15, 15), (25, 25))
        actual_rects = set(r1.remove_overlap_with(r2))
        expected_rects = {Rect((10, 10), (15, 15)), 
                          Rect((10, 15), (15, 20)), 
                          Rect((15, 10), (20, 15))}
        self.assertEqual(expected_rects, actual_rects)


def get_lines(filename):
    with open(filename) as f:
        return [line.strip() for line in f.readlines() if line]

if __name__ == '__main__':
    main()
