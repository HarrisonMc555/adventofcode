#!/usr/bin/env python3

import re

USE_EXAMPLE = False

MAX_NUM_MOVES = 10 if USE_EXAMPLE else 100

INPUT = '685974213'
EXAMPLE = '389125467'
TEXT = EXAMPLE if USE_EXAMPLE else INPUT

NUM_PICK_UP = 3

DEBUG = USE_EXAMPLE

def main():
    text = EXAMPLE if USE_EXAMPLE else INPUT
    labels = parse_text(text)
    first_node, node_map = create_nodes(labels, max(labels))
    for label in labels:
        debug_print(f'{label}: {node_map[label]}')
    play_game(first_node, node_map)
    values = start_at(node_map[1])
    print(''.join(str(value) for value in values[1:]))

def play_game(first_node, node_map):
    min_label, max_label = min(node_map), max(node_map)
    cur_node = first_node
    for move_num in range(1, MAX_NUM_MOVES + 1):
        debug_print(f'-- move {move_num} --')
        debug_print(f'cups: {cups_str(cur_node)}')
        picked_up = pick_up(cur_node, NUM_PICK_UP)
        picked_up[-1].next = picked_up[0]
        debug_print(f'pick up: {cups_str(picked_up[0])}')
        destination_label = cur_node.value - 1
        if destination_label < min_label:
            destination_label = max_label
        picked_up_values = {node.value for node in picked_up}
        while destination_label in picked_up_values:
            destination_label -= 1
            if destination_label < min_label:
                destination_label = max_label
        debug_print(f'destination_label: {destination_label}')
        node_map[destination_label].insert_after(picked_up)
        cur_node = cur_node.next
        debug_print()
    debug_print('-- final --')
    debug_print(f'cups: {cups_str(cur_node)}')

def start_at(first_node):
    result = [first_node.value]
    cur_node = first_node.next
    while cur_node.value != first_node.value:
        result.append(cur_node.value)
        cur_node = cur_node.next
    return result

def pick_up(node, num_after):
    result = []
    for _ in range(num_after):
        result.append(node.pop_next())
    return result

def cups_str(cur_node):
    result = []
    result.append(f'({cur_node.value})')
    first_node = cur_node
    cur_node = first_node.next
    while cur_node.value != first_node.value:
        result.append(f' {cur_node.value} ')
        cur_node = cur_node.next
    return ''.join(result)

def create_nodes(init_labels, max_label):
    debug_print(f'init_labels: {init_labels}')
    first_label = init_labels[0]
    first_node = Node(first_label)
    prev_node = first_node
    node_map = {first_label: first_node}
    debug_print(f'first_node: {first_node}')
    for value in init_labels[1:]:
        debug_print(f'value: {value} (from init_labels)')
        node = Node(value)
        node_map[value] = node
        prev_node.next = node
        debug_print(f'\tnode: {node}')
        debug_print(f'\tprev_node: {prev_node}')
        prev_node = node
    for value in range(max(init_labels) + 1, max_label + 1):
        debug_print(f'value: {value} (from extra)')
        node = Node(value)
        node_map[value] = node
        prev_node.next = node
        debug_print(f'\tnode: {node}')
        debug_print(f'\tprev_node: {prev_node}')
        prev_node = node
    prev_node.next = first_node
    debug_print(f'prev_node: {prev_node}')
    return first_node, node_map

class Node:
    def __init__(self, value):
        self.value = value
        self.next = None

    def __str__(self):
        next_value = self.next.value if self.next else 'N/A'
        return f'Node({self.value}) -> {next_value}'

    def pop_next(self):
        next_node = self.next
        next_next_node = next_node.next
        self.next = next_next_node
        return next_node

    def insert_after(self, nodes):
        original_next_node = self.next
        cur_node = self
        for node in nodes:
            cur_node.next = node
            cur_node = node
        cur_node.next = original_next_node

def parse_text(text):
    return [int(c) for c in text]

def debug_print(*args, **kwargs):
    if DEBUG:
        print(*args, **kwargs)

if __name__ == '__main__':
    main()
