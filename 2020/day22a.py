#!/usr/bin/env python3

import re

INPUT_FILE = 'input22.txt'
# INPUT_FILE = 'example22.txt'

DEBUG = False

def main():
    text = get_text(INPUT_FILE)
    deck1, deck2 = parse_text(text)
    debug_print(deck1)
    debug_print(deck2)
    winning_deck = play_game(deck1, deck2)
    print(calculate_score(winning_deck))

def play_game(deck1, deck2):
    round_num = 0
    while deck1 and deck2:
        round_num += 1
        if round_num > 1000:
            raise Exception('Game went on too long')
        debug_print(f'-- Round {round_num} --')
        debug_print(f'Player 1\'s deck: {deck_to_str(deck1)}')
        debug_print(f'Player 2\'s deck: {deck_to_str(deck2)}')
        card1 = deck1.pop(0)
        card2 = deck2.pop(0)
        debug_print(f'Player 1 plays: {card1}')
        debug_print(f'Player 2 plays: {card2}')
        p1won = card1 > card2
        debug_print(f'Player {"1" if p1won else "2"} wins the round!')
        debug_print()
        winning_card = card1 if p1won else card2
        losing_card = card2 if p1won else card1
        winning_deck = deck1 if p1won else deck2
        winning_deck.append(winning_card)
        winning_deck.append(losing_card)
    debug_print('== Post-game results ==')
    debug_print(f'Player 1\'s deck: {deck_to_str(deck1)}')
    debug_print(f'Player 2\'s deck: {deck_to_str(deck2)}')
    if not deck1:
        return deck2
    if not deck2:
        return deck1
    raise Exception('Game over but no winning deck')

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

def debug_print(*args, **kwargs):
    if DEBUG:
        print(*args, **kwargs)

if __name__ == '__main__':
    main()
