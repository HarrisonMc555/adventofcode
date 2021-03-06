#!/usr/bin/env python3

import re
from enum import Enum, auto

INPUT_FILE = 'input17.txt'
# INPUT_FILE = 'example17.txt'

NUM_BOOT_CYCLES = 6

def main():
    text = get_text(INPUT_FILE)
    grid = parse_text(text)
    # print_grid(grid)
    conway3d = Conway3D.from_grid(grid)
    # print(conway3d)
    # print()
    for i in range(NUM_BOOT_CYCLES):
        conway3d = conway3d.step()
        # print(f'After {i+1} steps')
        # print(conway3d)
        # print()
    print(conway3d.num_active())

def get_text(filename):
    with open(filename) as f:
        return f.read().strip()

def parse_text(text):
    return [parse_line(line.strip()) for line in text.split('\n')]

def parse_line(line):
    return [Conway3D.char_to_active(c) for c in line]

def print_grid(grid):
    for row in grid:
        for cube in row:
            print(cube, end='')
        print()

class Conway3D:
    def __init__(self, actives):
        if not actives:
            actives = set()
        self.actives = actives

    def from_grid(grid):
        actives = set()
        z = 0
        for y, row in enumerate(grid):
            for x, is_active in enumerate(row):
                if is_active:
                    actives.add((x, y, z))
        return Conway3D(actives)

    def step(self):
        next_actives = set()
        (min_x, max_x), (min_y, max_y), (min_z, max_z) = self.get_boundaries()
        for x in range(min_x - 1, max_x + 2):
            for y in range(min_y - 1, max_y + 2):
                for z in range(min_z - 1, max_z + 2):
                    if self.is_active_next(x, y, z):
                        next_actives.add((x, y, z))
        return Conway3D(next_actives)

    def num_active(self):
        return len(self.actives)

    def is_active_next(self, x, y, z):
        neighbor_indices = Conway3D.get_neighbor_indices(x, y, z)
        neighbors_is_active = [indices in self.actives for indices in neighbor_indices]
        num_active_neighbors = neighbors_is_active.count(True)
        if (x, y, z) in self.actives:
            return 2 <= num_active_neighbors <= 3
        else:
            return num_active_neighbors == 3

    def get_neighbor_indices(x, y, z):
        result = []
        for x2 in range(x - 1, x + 2):
            for y2 in range(y - 1, y + 2):
                for z2 in range(z - 1, z + 2):
                    if x == x2 and y == y2 and z == z2:
                        continue
                    result.append((x2, y2, z2))
        return result

    def __str__(self):
        lines = []

        (min_x, max_x), (min_y, max_y), (min_z, max_z) = self.get_boundaries()

        for z in range(min_z, max_z + 1):
            lines.append(f'z={z}')
            for y in range(min_y, max_y + 1):
                line = []
                for x in range(min_x, max_x + 1):
                    is_active = (x, y, z) in self.actives
                    line.append(Conway3D.active_to_char(is_active))
                lines.append(''.join(line))
            if z < max_z:
                lines.append('')

        return '\n'.join(lines)

    def get_boundaries(self):
        xs = [x for x, _, _ in self.actives]
        min_x, max_x = min(xs), max(xs)
        ys = [y for _, y, _ in self.actives]
        min_y, max_y = min(ys), max(ys)
        zs = [z for _, _, z in self.actives]
        min_z, max_z = min(zs), max(zs)
        return (min_x, max_x), (min_y, max_y), (min_z, max_z)

    def active_to_char(is_active):
        return '#' if is_active else '.'

    def char_to_active(c):
        return c == '#'

if __name__ == '__main__':
    main()
