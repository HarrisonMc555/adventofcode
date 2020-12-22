#!/usr/bin/env python3

import re

INPUT_FILE = 'input22.txt'
# INPUT_FILE = 'example22.txt'

DEBUG = False

INFINTE_GAME = '''Player 1:
43
19

Player 2:
2
29
14'''

MAX_GAMES = 100_000
MAX_ROUNDS = 100_000

def main():
    text = get_text(INPUT_FILE)
    # text = INFINTE_GAME
    deck1, deck2 = parse_text(text)
    p1won = play_game(deck1, deck2)
    winning_deck = deck1 if p1won else deck2
    print(calculate_score(winning_deck))

GLOBAL_GAME_NUM = 1
def play_game(deck1, deck2):
    global GLOBAL_GAME_NUM
    GLOBAL_GAME_NUM = 1
    p1won = play_game_helper(deck1, deck2, GLOBAL_GAME_NUM)
    debug_print()
    debug_print()
    debug_print('== Post-game results ==')
    debug_print(f'Player {p1won_to_str(p1won)} won!')
    debug_print(f'Player 1\'s deck: {deck_to_str(deck1)}')
    debug_print(f'Player 2\'s deck: {deck_to_str(deck2)}')
    return p1won

def play_game_helper(deck1, deck2, game_num):
    global GLOBAL_GAME_NUM
    if game_num > MAX_GAMES:
        raise Exception('Game went on too long')
    debug_print(f'== Game {game_num} ==')
    round_num = 0
    seen = set()
    while deck1 and deck2:
        round_num += 1
        if round_num > MAX_ROUNDS:
            raise Exception('Round went on too long')
        debug_print()
        debug_print(f'-- Round {round_num} (Game {game_num}) --')
        debug_print(f'Player 1\'s deck: {deck_to_str(deck1)}')
        debug_print(f'Player 2\'s deck: {deck_to_str(deck2)}')
        state = (tuple(deck1), tuple(deck2))
        if state in seen:
            debug_print('Infinite game detected, player 1 wins!')
            return True
        seen.add(state)
        card1 = deck1.pop(0)
        card2 = deck2.pop(0)
        debug_print(f'Player 1 plays: {card1}')
        debug_print(f'Player 2 plays: {card2}')
        if card1 <= len(deck1) and card2 <= len(deck2):
            debug_print('Playing a sub-game to determine the winner...')
            debug_print()
            GLOBAL_GAME_NUM += 1
            p1won = play_game_helper(deck1[:card1], deck2[:card2], GLOBAL_GAME_NUM)
            debug_print()
            debug_print(f'...anyway, back to game {game_num}.')
        else:
            p1won = card1 > card2
        debug_print(f'Player {p1won_to_str(p1won)} wins round {round_num} of game {game_num}!')
        winning_card = card1 if p1won else card2
        losing_card = card2 if p1won else card1
        winning_deck = deck1 if p1won else deck2
        winning_deck.append(winning_card)
        winning_deck.append(losing_card)
    p1won = len(deck1) > 0
    debug_print(f'The winner of game {game_num} is player {p1won_to_str(p1won)}!')
    return p1won

def calculate_score(deck):
    scores = [card * (i + 1) for i, card in enumerate(reversed(deck))]
    return sum(scores)

def parse_text(text):
    deck1_text, deck2_text = text.split('\n\n')
    return parse_deck(deck1_text), parse_deck(deck2_text)

def parse_deck(text):
    lines = text.split('\n')
    deck_lines = lines[1:]
    return [int(x) for x in deck_lines]

def get_text(filename):
    with open(filename) as f:
        return f.read().strip()

def deck_to_str(deck):
    return ', '.join(str(x) for x in deck)

def p1won_to_str(p1won):
    return '1' if p1won else '2'

def debug_print(*args, **kwargs):
    if DEBUG:
        print(*args, **kwargs)

if __name__ == '__main__':
    main()
