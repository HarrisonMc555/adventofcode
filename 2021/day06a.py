#!/usr/bin/env python3

INPUT_FILE = 'input06.txt'
# INPUT_FILE = 'example06.txt'

NUM_GENERATIONS = 80
CYCLE_LENGTH = 7

from collections import defaultdict

def main():
    text = get_text(INPUT_FILE)
    nums = parse_text(text)
    lanternfish_final = simulate(nums, NUM_GENERATIONS)
    print(sum(lanternfish_final.values()))

def simulate(nums, num_generations):
    timers = defaultdict(int)
    for num in nums:
        timers[num] += 1
    for _ in range(num_generations):
        new_timers = defaultdict(int)
        for timer, count in timers.items():
            if timer > 0:
                new_timers[timer - 1] += count
            else:
                new_timers[CYCLE_LENGTH - 1] += count
                new_timers[CYCLE_LENGTH + 1] += count
        timers = new_timers
    return timers

def parse_text(text):
    return [int(word) for word in text.split(',')]

def get_text(filename):
    with open(filename) as f:
        return f.read().strip()

if __name__ == '__main__':
    main()
