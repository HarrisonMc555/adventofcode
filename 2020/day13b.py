#!/usr/bin/env python3

import unittest

INPUT_FILE = 'input13.txt'
# INPUT_FILE = 'example.txt'

def main():
    # unittest.main()
    lines = get_lines(INPUT_FILE)
    bus_infos = parse_lines(lines)
    if not all(is_prime(bus_id) for bus_id, _ in bus_infos):
        print('Algorithm invalid if bus ids are not all prime')
        return
    time = find_earliest_time(bus_infos)
    print(time)

def get_lines(filename):
    with open(filename) as f:
        return [line.strip() for line in f.readlines() if line]

def parse_lines(lines):
    return parse_text(lines[1])

def parse_text(text):
    bus_strings = text.split(',')
    return [(int(s), offset)
            for offset, s in enumerate(bus_strings)
            if s != 'x']

def find_earliest_time(bus_infos):
    timestamp = 0
    period = 1
    for bus_id, offset in bus_infos:
        timestamp, period = find_next_timestamp_period(
            timestamp, period, bus_id, offset)
    return timestamp

def find_next_timestamp_period(timestamp, period, bus_id, offset):
    remainder = get_remainder(bus_id, offset)
    new_timestamp = timestamp
    while new_timestamp % bus_id != remainder:
        new_timestamp += period
    new_period = period * bus_id
    return new_timestamp, new_period

def get_remainder(bus_id, minutes_after):
    return (bus_id - minutes_after) % bus_id

def is_prime(num):
    return all(num % divisor != 0 for divisor in range(2, num))
        
class Part2Test(unittest.TestCase):
    def test_examples(self):
        cases = [
            ("17,x,13,19", 3417),
            ("67,7,59,61", 754018),
            ("67,x,7,59,61", 779210),
            ("67,7,x,59,61", 1261476),
            ("1789,37,47,1889", 1202161486),
        ]
        for text, expected in cases:
            bus_infos = parse_text(text)
            self.assertEqual(expected, find_earliest_time(bus_infos))

if __name__ == '__main__':
    main()
