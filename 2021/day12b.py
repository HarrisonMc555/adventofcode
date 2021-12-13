#!/usr/bin/env python3

# INPUT_FILE = 'input12.txt'

import unittest
from collections import defaultdict

def main():
    if 'INPUT_FILE' not in globals():
        unittest.main()
        return
    lines = get_lines(INPUT_FILE)
    pairs = parse_lines(lines)
    paths = get_paths(pairs)
    print(len(paths))

START = 'start'
END = 'end'
def get_paths(pairs):
    links = get_links(pairs)
    cur_path = [START]
    seen = {START}
    return get_paths_helper(links, cur_path, seen, None)

def get_paths_helper(links, cur_path, seen, twice_small_cave):
    cur_loc = cur_path[-1]
    if cur_loc == END:
        return [cur_path]
    results = []
    new_seen = {loc for loc in seen}
    new_seen.add(cur_loc)
    for next_loc in links[cur_loc]:
        if next_loc in seen and not is_large_cave(next_loc):
            if twice_small_cave or next_loc == START or next_loc == END:
                continue
            else:
                results.extend(get_paths_helper(
                    links, cur_path + [next_loc], new_seen, next_loc))
        else:
            results.extend(get_paths_helper(
                links, cur_path + [next_loc], new_seen, twice_small_cave))
    return results

def get_links(pairs):
    links = defaultdict(list)
    for from_loc, to_loc in pairs:
        links[from_loc].append(to_loc)
        links[to_loc].append(from_loc)
    return links

def is_large_cave(loc):
    return loc.isupper()

def parse_lines(lines):
    return [parse_line(line) for line in lines]

def parse_line(line):
    return tuple(line.split('-'))

def get_lines(filename):
    with open(filename) as f:
        return [line.strip() for line in f.readlines()]

SMALL_LINES = [
    'start-A',
    'start-b',
    'A-c',
    'A-b',
    'b-d',
    'A-end',
    'b-end',
]
SMALL_PATHS = {
    'start,A,b,A,b,A,c,A,end',
    'start,A,b,A,b,A,end',
    'start,A,b,A,b,end',
    'start,A,b,A,c,A,b,A,end',
    'start,A,b,A,c,A,b,end',
    'start,A,b,A,c,A,c,A,end',
    'start,A,b,A,c,A,end',
    'start,A,b,A,end',
    'start,A,b,d,b,A,c,A,end',
    'start,A,b,d,b,A,end',
    'start,A,b,d,b,end',
    'start,A,b,end',
    'start,A,c,A,b,A,b,A,end',
    'start,A,c,A,b,A,b,end',
    'start,A,c,A,b,A,c,A,end',
    'start,A,c,A,b,A,end',
    'start,A,c,A,b,d,b,A,end',
    'start,A,c,A,b,d,b,end',
    'start,A,c,A,b,end',
    'start,A,c,A,c,A,b,A,end',
    'start,A,c,A,c,A,b,end',
    'start,A,c,A,c,A,end',
    'start,A,c,A,end',
    'start,A,end',
    'start,b,A,b,A,c,A,end',
    'start,b,A,b,A,end',
    'start,b,A,b,end',
    'start,b,A,c,A,b,A,end',
    'start,b,A,c,A,b,end',
    'start,b,A,c,A,c,A,end',
    'start,b,A,c,A,end',
    'start,b,A,end',
    'start,b,d,b,A,c,A,end',
    'start,b,d,b,A,end',
    'start,b,d,b,end',
    'start,b,end',
}

MEDIUM_LINES = [
    'dc-end',
    'HN-start',
    'start-kj',
    'dc-start',
    'dc-HN',
    'LN-dc',
    'HN-end',
    'kj-sa',
    'kj-HN',
    'kj-dc',
]
NUM_MEDIUM_PATHS = 103

LARGE_LINES = [
    'fs-end',
    'he-DX',
    'fs-he',
    'start-DX',
    'pj-DX',
    'end-zg',
    'zg-sl',
    'zg-pj',
    'pj-he',
    'RW-he',
    'fs-DX',
    'pj-RW',
    'zg-RW',
    'start-pj',
    'he-WI',
    'zg-he',
    'pj-fs',
    'start-RW',
]
NUM_LARGE_PATHS = 3509

class Test(unittest.TestCase):
    def test_small(self):
        paths = get_paths(parse_lines(SMALL_LINES))
        formatted_paths = {','.join(path) for path in paths}
        self.assertEqual(formatted_paths, SMALL_PATHS)

    def test_medium(self):
        paths = get_paths(parse_lines(MEDIUM_LINES))
        num_paths = len(paths)
        self.assertEqual(num_paths, NUM_MEDIUM_PATHS)

    def test_large(self):
        paths = get_paths(parse_lines(LARGE_LINES))
        num_paths = len(paths)
        self.assertEqual(num_paths, NUM_LARGE_PATHS)


if __name__ == '__main__':
    main()
