#!/usr/bin/env python3

from enum import Enum, auto
from dataclasses import dataclass, field
from collections import Counter
import itertools

# TEST = True

REAL = False
REAL = True

if REAL:
    STARTING_POSITIONS = 1, 3
else:
    STARTING_POSITIONS = 4, 8
import unittest
import re

DEBUG = False
# DEBUG = True
def debug_print(*args, **kwargs):
    if DEBUG:
        print(*args, **kwargs)

MAX_POINTS = 21
def main():
    if 'TEST' in globals():
        unittest.main()
        return
    print(run(STARTING_POSITIONS, MAX_POINTS))

def run(starting_positions, max_points):
    state_to_counts = {}
    state = State.create(*starting_positions)
    wins_p1, wins_p2 = get_counts(state_to_counts, state, max_points)
    return max(wins_p1, wins_p2)

DICE_SUM_TO_COUNT = Counter(sum(p) for p in
                            itertools.product(range(1, 3 + 1), repeat=3))
def get_counts(state_to_counts, state, max_points):
    if state in state_to_counts:
        return state_to_counts[state]
    winning_player = state.winning_player(max_points)
    counts = None
    if winning_player == Player.P1:
        counts = 1, 0
    elif winning_player == Player.P2:
        counts = 0, 1
    if counts:
        state_to_counts[state] = counts
        return counts
    total_wins_p1, total_wins_p2 = 0, 0
    for dice_sum, count in DICE_SUM_TO_COUNT.items():
        next_state = state.play_turn(dice_sum)
        wins_p1, wins_p2 = get_counts(state_to_counts, next_state, max_points)
        total_wins_p1 += wins_p1 * count
        total_wins_p2 += wins_p2 * count
    counts = total_wins_p1, total_wins_p2
    state_to_counts[state] = counts
    return counts

def wrap_pos(pos):
    return (pos - 1) % 10 + 1

class Player(Enum):
    P1 = auto()
    P2 = auto()

    def other_player(self):
        if self == Player.P1:
            return Player.P2
        else:
            return Player.P1

@dataclass(frozen=True)
class PlayerInfo:
    pos: int
    score: int = field(default=0)

    def play_turn(self, dice_sum):
        pos = wrap_pos(self.pos + dice_sum)
        score = self.score + pos
        return PlayerInfo(pos, score)

@dataclass(frozen=True)
class State:
    p1: PlayerInfo
    p2: PlayerInfo
    cur_player: Player = field(default=Player.P1)

    @staticmethod
    def create(p1pos, p2pos):
        return State(PlayerInfo(p1pos), PlayerInfo(p2pos))

    def get_player_info(self, player):
        if player == Player.P1:
            return self.p1
        else:
            return self.p2

    def winning_player(self, max_points):
        if self.p1.score >= max_points:
            return Player.P1
        if self.p2.score >= max_points:
            return Player.P2
        return None

    def play_turn(self, dice_sum):
        next_player = self.cur_player.other_player()
        if self.cur_player == Player.P1:
            return State(self.p1.play_turn(dice_sum), self.p2, next_player)
        else:
            return State(self.p1, self.p2.play_turn(dice_sum), next_player)

class Test(unittest.TestCase):
    def test_wrap_pos(self):
        self.assertEqual(wrap_pos(4 + 1 + 2 + 3), 10)
        self.assertEqual(wrap_pos(8 + 4 + 5 + 6), 3)
        self.assertEqual(wrap_pos(10 + 7 + 8 + 9), 4)
        self.assertEqual(wrap_pos(3 + 10 + 11 + 12), 6)
        self.assertEqual(wrap_pos(4 + 13 + 14 + 15), 6)

    def test_play_turn(self):
        state = State.create(4, 8)
        self.assertEqual(state.p1, PlayerInfo(4, 0))
        self.assertEqual(state.p2, PlayerInfo(8, 0))
        self.assertEqual(state.cur_player, Player.P1)
        state = state.play_turn(1 + 2 + 3)
        self.assertEqual(state.p1, PlayerInfo(10, 10))
        self.assertEqual(state.p2, PlayerInfo(8, 0))
        self.assertEqual(state.cur_player, Player.P2)
        state = state.play_turn(4 + 5 + 6)
        self.assertEqual(state.p1, PlayerInfo(10, 10))
        self.assertEqual(state.p2, PlayerInfo(3, 3))
        self.assertEqual(state.cur_player, Player.P1)
        state = state.play_turn(7 + 8 + 9)
        self.assertEqual(state.p1, PlayerInfo(4, 14))
        self.assertEqual(state.p2, PlayerInfo(3, 3))
        self.assertEqual(state.cur_player, Player.P2)
        state = state.play_turn(10 + 11 + 12)
        self.assertEqual(state.p1, PlayerInfo(4, 14))
        self.assertEqual(state.p2, PlayerInfo(6, 9))
        self.assertEqual(state.cur_player, Player.P1)

    def test_wining_player(self):
        state = State.create(4, 8)
        self.assertEqual(state.winning_player(21), None)
        self.assertEqual(state.winning_player(1000), None)
        state = State(PlayerInfo(6, 26), PlayerInfo(7, 16))
        self.assertEqual(state.winning_player(21), Player.P1)
        self.assertEqual(state.winning_player(1000), None)

    def test_run(self):
        max_wins = run((4, 8), 1)
        num_futures_each_roll = 3**3
        self.assertEqual(max_wins, num_futures_each_roll)

    def test_get_counts(self):
        num_futures_each_roll = 3**3
        wins_p1, wins_p2 = get_counts({}, State.create(4, 8), 1)
        self.assertEqual(wins_p1, num_futures_each_roll)
        self.assertEqual(wins_p2, 0)

        info_p1 = PlayerInfo(1, 90)
        info_p2 = PlayerInfo(1, 99)
        state = State(info_p1, info_p2, Player.P1)
        wins_p1, wins_p2 = get_counts({}, state, 100)
        # land on ...               10
        # roll a ...                9
        num_futures_where_p1_wins = 1
        num_futures_p2_rolls = num_futures_each_roll - num_futures_where_p1_wins
        num_futures_p2_wins = num_futures_p2_rolls * num_futures_each_roll
        self.assertEqual(wins_p1, num_futures_where_p1_wins)
        self.assertEqual(wins_p2, num_futures_p2_wins)

        info_p1 = PlayerInfo(1, 91)
        info_p2 = PlayerInfo(1, 99)
        state = State(info_p1, info_p2, Player.P1)
        wins_p1, wins_p2 = get_counts({}, state, 100)
        # land on ...               9   10
        # roll a ...                8   9
        num_futures_where_p1_wins = 3 + 1
        num_futures_p2_rolls = num_futures_each_roll - num_futures_where_p1_wins
        num_futures_p2_wins = num_futures_p2_rolls * num_futures_each_roll
        self.assertEqual(wins_p1, num_futures_where_p1_wins)
        self.assertEqual(wins_p2, num_futures_p2_wins)

if __name__ == '__main__':
    main()
