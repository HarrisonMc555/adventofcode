#!/usr/bin/env python3

import re

INPUT = '685974213'
EXAMPLE = '389125467'

MAX_NUM_MOVES = 100
# MAX_NUM_MOVES = 10
NUM_PICK_UP = 3

DEBUG = False
# DEBUG = True

def main():
    labels = parse_text(INPUT)
    # labels = parse_text(EXAMPLE)
    play_game(labels)
    start_at_1 = start_at(labels, 1)
    print(''.join(str(x) for x in start_at_1[1:]))

def play_game(labels):
    cur_index = 0
    all_labels = set(labels)
    min_label, max_label = min(labels), max(labels)
    for move_num in range(1, MAX_NUM_MOVES + 1):
        cur_label = labels[cur_index]
        debug_print(f'-- move {move_num} --')
        debug_print(f'cups: {cups_str(labels, cur_label)}')
        picked_up, num_from_beginning = pick_up(labels, cur_index + 1, NUM_PICK_UP)
        # picked_up = labels[cur_index + 1:cur_index + 1 + NUM_PICK_UP]
        # del labels[cur_index + 1:cur_index + 1 + NUM_PICK_UP]
        debug_print(f'pick up: {cups_str(picked_up, None)}')
        debug_print(f'\tcups: {cups_str(labels, cur_label)}')
        debug_print(f'\tnum_from_beginning: {num_from_beginning}')
        cur_index -= num_from_beginning
        debug_print(f'\tcur_index: {cur_index}')
        destination_label = cur_label - 1
        if destination_label < min_label:
            destination_label = max_label
        while destination_label in picked_up:
            debug_print(f'\twe picked up {destination_label}, ', end='')
            destination_label -= 1
            if destination_label < min_label:
                destination_label = max_label
            debug_print(f'try {destination_label}')
        debug_print(f'destination: {destination_label}')
        destination_index = labels.index(destination_label)
        debug_print(f'\tdestination_index: {destination_index}')
        labels[destination_index + 1:destination_index + 1] = picked_up
        diff = 1 if cur_index < destination_index else NUM_PICK_UP + 1
        cur_index = (cur_index + diff) % len(labels)
        debug_print()
    debug_print('-- final --')
    cur_label = labels[cur_index]
    debug_print(f'cups: {cups_str(labels, cur_label)}')

def pick_up(labels, index, num):
    result = []
    num_from_beginning = 0
    while index < len(labels) and len(result) < num:
        result.append(labels.pop(index))
    while len(result) < num:
        result.append(labels.pop(0))
        num_from_beginning += 1
    return result, num_from_beginning

def start_at(labels, label):
    index = labels.index(label)
    return labels[index:] + labels[:index]

def parse_text(text):
    return [int(c) for c in text]

def cups_str(labels, cur_label):
    return ''.join(cup_str(label, cur_label) for label in labels)

def cup_str(label, cur_label):
    if label == cur_label:
        return f'({label})'
    else:
        return f' {label} '

def debug_print(*args, **kwargs):
    if DEBUG:
        print(*args, **kwargs)

if __name__ == '__main__':
    main()
