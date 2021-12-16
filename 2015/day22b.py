#!/usr/bin/env python3

INPUT_FILE = 'input22.txt'

import unittest
import math
from dataclasses import dataclass
from typing import Callable
from enum import Enum, auto
from copy import deepcopy

DEBUG = False
def debug_print(*args, **kwargs):
    if DEBUG:
        print(*args, **kwargs)

def main():
    if 'INPUT_FILE' not in globals():
        unittest.main()
        return
    player_stats = 50, 500
    boss_stats = 55, 8
    print(get_least_mana(player_stats, boss_stats))

def get_least_mana(player_stats, boss_stats):
    games = [Game.create(player_stats, boss_stats)]
    lowest_mana = math.inf
    count = 0
    while games:
        count += 1
        next_games = []
        for game in games:
            if game.total_mana > lowest_mana:
                continue
            for action in PLAYER_ACTIONS:
                next_game = deepcopy(game)
                result = next_game.run_turn(action)
                if result == TurnResult.WIN:
                    debug_print(f'We won! {next_game}')
                    lowest_mana = min(lowest_mana, next_game.total_mana)
                elif result == TurnResult.LOSE:
                    debug_print('Lost.')
                    pass
                elif result == TurnResult.CONTINUE:
                    next_games.append(next_game)
                else:
                    raise Exception('Non-exhaustive switch')
        games = next_games
    return lowest_mana

@dataclass
class Game:
    player_hit_points: int
    player_mana: int
    boss_hit_points: int
    boss_damage: int
    shield_turns: int
    poison_turns: int
    recharge_turns: int
    total_mana: int
    actions: [Callable[[int], str]]

    @staticmethod
    def create(player_stats, boss_stats):
        player_hit_points, player_mana = player_stats
        boss_hit_points, boss_damage = boss_stats
        return Game(player_hit_points, player_mana, boss_hit_points, boss_damage, 0, 0, 0, 0, [])

    def summary(self):
        # action_string = ''.join(a.__name__[0] for a in self.actions)
        return f'Player: {self.player_hit_points:2}/10. ' + \
            f'Boss: {self.boss_hit_points:2}. ' + \
            f'Mana: {self.player_mana:3}. ' + \
            f'Actions: {self.action_string()}.'

    def action_string(self):
        return ''.join(action.__name__[0] for action in self.actions)

    def run_turn(self, action):
        self.actions.append(action)
        player_turn_result = self.player_turn(action)
        if player_turn_result != TurnResult.CONTINUE:
            return player_turn_result
        boss_turn_result = self.boss_turn()
        if boss_turn_result != TurnResult.CONTINUE:
            return boss_turn_result
        return TurnResult.CONTINUE

    def player_turn(self, action):
        debug_print('-- Player turn -- ')
        debug_print('Player loses 1 hit point from hard mode.', end='')
        self.player_hit_points -= 1
        if self.player_hit_points <= 0:
            debug_print(' This kills the player, and the player loses.')
            return TurnResult.LOSE
        else:
            debug_print()
        return self.turn(action)

    def boss_turn(self):
        debug_print('-- Boss turn --')
        return self.turn(Game.boss_attack)

    def turn(self, action):
        self.debug_print_info()
        result = self.apply_effects()
        if result != TurnResult.CONTINUE:
            return result
        result = action(self)
        debug_print()
        return result

    def magic_missile(self):
        mana = 53
        if self.player_mana < mana:
            debug_print(f'Need {mana} mana, only have {self.player_mana}. The player loses.')
            return TurnResult.LOSE
        self.player_mana -= mana
        self.total_mana += mana
        self.boss_hit_points -= 4
        debug_print('Player casts Magic Missile, dealing 4 damage.', end='')
        if self.boss_hit_points <= 0:
            debug_print(' This kills the boss, and the player wins.')
            return TurnResult.WIN
        else:
            debug_print()
        return TurnResult.CONTINUE

    def drain(self):
        mana = 73
        if self.player_mana < mana:
            debug_print(f'Need {mana} mana, only have {self.player_mana}. The player loses.')
            return TurnResult.LOSE
        self.player_mana -= mana
        self.total_mana += mana
        self.boss_hit_points -= 2
        self.player_hit_points += 2
        debug_print('Player casts Drain, dealing 2 damage, and healing 2 hit points.', end='')
        if self.boss_hit_points <= 0:
            debug_print(' This kills the boss, and the player wins.')
            return TurnResult.WIN
        else:
            debug_print()
        return TurnResult.CONTINUE

    def shield(self):
        mana = 113
        if self.shield_turns > 0:
            debug_print(f'Shield is already active with timer of {self.shield_turns}. The player loses.')
            return TurnResult.LOSE
        if self.player_mana < mana:
            debug_print(f'Need {mana} mana, only have {self.player_mana}. The player loses.')
            return TurnResult.LOSE
        debug_print('Player casts Shield, increasing armor by 7.')
        self.player_mana -= mana
        self.total_mana += mana
        self.shield_turns = 6
        return TurnResult.CONTINUE

    def poison(self):
        mana = 173
        if self.poison_turns > 0:
            debug_print(f'Poison is already active with timer of {self.poison_turns}. The player loses.')
            return TurnResult.LOSE
        if self.player_mana < mana:
            debug_print(f'Need {mana} mana, only have {self.player_mana}. The player loses.')
            return TurnResult.LOSE
        debug_print('Player casts Poison.')
        self.player_mana -= mana
        self.total_mana += mana
        self.poison_turns = 6
        return TurnResult.CONTINUE

    def recharge(self):
        mana = 229
        if self.recharge_turns > 0:
            debug_print(f'Recharge is already active with timer of {self.recharge_turns}. The player loses.')
            return TurnResult.LOSE
        if self.player_mana < mana:
            debug_print(f'Need {mana} mana, only have {self.player_mana}. The player loses.')
            return TurnResult.LOSE
        debug_print('Player casts Recharge.')
        self.player_mana -= mana
        self.total_mana += mana
        self.recharge_turns = 5
        return TurnResult.CONTINUE

    def boss_attack(self):
        damage_to_player = self.calc_damage_to_player()
        self.player_hit_points -= damage_to_player
        if self.player_hit_points <= 0:
            debug_print(f'Boss attacks for {damage_to_player} damage. This kills the player, and the player loses.')
            return TurnResult.LOSE
        else:
            debug_print(f'Boss attacks for {damage_to_player} damage!')
            return TurnResult.CONTINUE

    def calc_damage_to_player(self):
        difference = self.boss_damage - self.calc_player_armor()
        if difference <= 0:
            return 1
        else:
            return difference

    def calc_player_armor(self):
        if self.shield_turns:
            return 7
        else:
            return 0

    def apply_effects(self):
        if self.shield_turns:
            self.shield_turns -= 1
            debug_print(f'Shield\'s timer is now {self.shield_turns}.')
            if not self.shield_turns:
                debug_print(f'Shield wears off.')
        if self.poison_turns:
            self.boss_hit_points -= 3
            self.poison_turns -= 1
            if self.boss_hit_points <= 0:
                debug_print(f'Poison deals 3 damage. This kills the boss, and the player wins.')
                return TurnResult.WIN
            else:
                debug_print(f'Poison deals 3 damage; its timer is now {self.poison_turns}.')
            if not self.poison_turns:
                debug_print(f'Poison wears off.')
        if self.recharge_turns:
            self.player_mana += 101
            self.recharge_turns -= 1
            debug_print(f'Recharge provides 101 mana; its timer is now {self.recharge_turns}.')
            if not self.recharge_turns:
                debug_print(f'Recharge wears off.')
        return TurnResult.CONTINUE

    def debug_print_info(self):
        debug_print(f'- Player has {self.player_hit_points} hit points, ' +
                    f'{self.calc_player_armor()} armor, ' +
                    f'{self.player_mana} mana, ' +
                    f'has spent {self.total_mana} total mana')
        debug_print(f'- Boss has {self.boss_hit_points} hit points')

PLAYER_ACTIONS = [
    Game.magic_missile,
    Game.drain,
    Game.shield,
    Game.poison,
    Game.recharge,
]

class TurnResult(Enum):
    WIN = auto()
    LOSE = auto()
    CONTINUE = auto()

def calc_damage(damage, armor):
    difference = damage - armor
    if difference <= 0:
        return 1
    else:
        return difference

def parse_boss_lines(lines):
    return [parse_boss_line(line) for line in lines]

def parse_boss_line(line):
    return int(line.split(' ')[-1])

def get_lines(filename):
    with open(filename) as f:
        return [line.strip() for line in f.readlines()]

class Test(unittest.TestCase):
    def test_example1(self):
        player_stats = 10, 250
        boss_stats = 13, 8
        actions = [
            Game.poison,
            Game.magic_missile,
        ]
        result = self.run_example(player_stats, boss_stats, actions)
        self.assertEqual(result, TurnResult.WIN)

    def test_example2(self):
        player_stats = 10, 250
        boss_stats = 14, 8
        actions = [
            Game.recharge,
            Game.shield,
            Game.drain,
            Game.poison,
            Game.magic_missile
        ]
        result = self.run_example(player_stats, boss_stats, actions)
        self.assertEqual(result, TurnResult.WIN)

    def test_real1(self):
        player_stats = 50, 500
        boss_stats = 55, 8
        actions = [
            Game.magic_missile,
            Game.poison,
            Game.recharge,
            Game.magic_missile,
            Game.shield,
            Game.poison,
            Game.magic_missile,
            Game.magic_missile,
            Game.magic_missile,
        ]
        # global DEBUG
        # DEBUG = True
        result = self.run_example(player_stats, boss_stats, actions)
        # DEBUG = False
        self.assertEqual(result, TurnResult.WIN)

    def run_example(self, player_stats, boss_stats, actions):
        game = Game.create(player_stats, boss_stats)
        for action in actions:
            result = game.run_turn(action)
            if result != TurnResult.CONTINUE:
                return result

if __name__ == '__main__':
    main()
