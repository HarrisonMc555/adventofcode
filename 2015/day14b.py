#!/usr/bin/env python3

INPUT_FILE = 'input14.txt'

from dataclasses import dataclass
import unittest
import re

DEBUG = True
def debug_print(*args, **kwargs):
    if DEBUG:
        print(*args, **kwargs)

NUM_SECONDS = 2503
def main():
    if 'INPUT_FILE' not in globals():
        unittest.main()
        return
    lines = get_lines(INPUT_FILE)
    reindeer_list = parse_lines(lines)
    run_game(reindeer_list, NUM_SECONDS)
    print(max(reindeer.score for reindeer in reindeer_list))

def run_game(reindeer_list, num_steps):
    for _ in range(num_steps):
        run_step(reindeer_list)

def run_step(reindeer_list):
    for reindeer in reindeer_list:
        reindeer.step()
    max_distance = max(r.distance for r in reindeer_list)
    for reindeer in reindeer_list:
        if reindeer.distance == max_distance:
            reindeer.score += 1

@dataclass
class Reindeer:
    name: str
    speed: int
    fly_time: int
    rest_time: int
    distance: int
    is_flying: bool
    cur_time: int
    score: int

    @staticmethod
    def new(name, speed, fly_time, rest_time):
        return Reindeer(name, speed, fly_time, rest_time, 0, True, 0, 0)

    def step(self):
        if self.is_flying:
            self.distance += self.speed

        self.cur_time += 1
        if (self.is_flying and self.cur_time >= self.fly_time) or \
           (not self.is_flying and self.cur_time >= self.rest_time):
            self.is_flying ^= True
            self.cur_time = 0


def parse_lines(lines):
    return [Reindeer.new(*parse_line(line)) for line in lines]

LINE_RE = re.compile(r'(\w+) can fly (\d+) km/s for (\d+) seconds, but then must rest for (\d+) seconds\.')
def parse_line(line):
    try:
        reindeer, speed, fly_time, rest_time = LINE_RE.match(line).groups()
    except Exception as e:
        print(f'Could not parse line: "{line}"')
        raise e
    return reindeer, int(speed), int(fly_time), int(rest_time)

def get_lines(filename):
    with open(filename) as f:
        return [line.strip() for line in f.readlines()]

EXAMPLE_LINES = [
    'Comet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds.',
    'Dancer can fly 16 km/s for 11 seconds, but then must rest for 162 seconds.',
]
TIME_TO_REINDEER_DISTANCES = {
    1: [('Comet', 14), ('Dancer', 16)],
    10: [('Comet', 140), ('Dancer', 160)],
    11: [('Comet', 140), ('Dancer', 176)],
    12: [('Comet', 140), ('Dancer', 176)],
    138: [('Comet', 154), ('Dancer', 176)],
    174: [('Dancer', 192)],
    1000: [('Comet', 1120), ('Dancer', 1056)],
}
class Test(unittest.TestCase):
    def test_examples(self):
        reindeer_list = parse_lines(EXAMPLE_LINES)
        run_game(reindeer_list, 1000)
        dancer = [r for r in reindeer_list if r.name == 'Dancer'][0]
        comet = [r for r in reindeer_list if r.name == 'Comet'][0]
        self.assertEqual(dancer.score, 689)
        self.assertEqual(comet.score, 312)

if __name__ == '__main__':
    main()
