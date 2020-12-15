#!/usr/bin/env python3

import unittest

STARTING_NUMS = [8, 11, 0, 19, 1, 2]

def main():
    # unittest.main()
    print(MemoryGame.nth(STARTING_NUMS, 2020))

class MemoryGame:
    def __init__(self, starting_nums):
        self.num_to_turn = {}
        for turn, num in enumerate(starting_nums[:-1]):
            self.num_to_turn[num] = turn
        self.prev_number = starting_nums[-1]
        self.turn = len(starting_nums) - 1
        self.index = 0

    def step(self):
        if self.prev_number in self.num_to_turn:
            prev_turn = self.num_to_turn[self.prev_number]
        else:
            prev_turn = self.turn
        next_number = self.turn - prev_turn
        self.num_to_turn[self.prev_number] = self.turn
        self.turn += 1
        self.prev_number = next_number
        return next_number

    def nth(starting_nums, n):
        game = MemoryGame(starting_nums)
        for _ in range(n - len(starting_nums) - 1):
            game.step()
        return game.step()

    def __str__(self):
        return f'turn = {self.turn}, prev_number = {self.prev_number}, ' + \
            f'num_to_turn = {self.num_to_turn}'

class MemoryTests(unittest.TestCase):
    def test_long_example(self):
        starting_nums = [0, 3, 6]
        game = MemoryGame(starting_nums)
        expected_nums = [0, 3, 6, 0, 3,  3, 1, 0, 4, 0]
        for num in expected_nums[len(starting_nums):]:
            self.assertEqual(num, game.step())

    def test_2020(self):
        test_cases = [
            ([1, 3, 2], 1),
            ([2, 1, 3], 10),
            ([1, 2, 3], 27),
            ([2, 3, 1], 78),
            ([3, 2, 1], 438),
            ([3, 1, 2], 1836),
        ]
        for starting_nums, n_2020 in test_cases:
            self.assertEqual(n_2020, MemoryGame.nth(starting_nums, 2020))

if __name__ == '__main__':
    main()
