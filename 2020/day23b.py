#!/usr/bin/env python3

import re

USE_EXAMPLE = False

MAX_NUM_MOVES = 10 if USE_EXAMPLE else 10_000_000
MAX_LABEL = 9 if USE_EXAMPLE else 1_000_000

INPUT = '685974213'
EXAMPLE = '389125467'
TEXT = EXAMPLE if USE_EXAMPLE else INPUT

NUM_PICK_UP = 3

DEBUG = USE_EXAMPLE

def main():
    text = EXAMPLE if USE_EXAMPLE else INPUT
    labels = parse_text(text)
    first_node, node_map = create_nodes(labels, MAX_LABEL)
    play_game(first_node, node_map)
    node1 = node_map[1]
    next_node = node1.next
    next_next_node = next_node.next
    print(next_node.value * next_next_node.value)

def play_game(first_node, node_map):
    min_label, max_label = min(node_map), max(node_map)
    cur_node = first_node
    for move_num in range(1, MAX_NUM_MOVES + 1):
        if move_num % 100_000 == 0:
            print(f'move_num: {move_num} ({move_num * 100 / MAX_NUM_MOVES}%)')
        picked_up = pick_up(cur_node, NUM_PICK_UP)
        picked_up[-1].next = picked_up[0]
        destination_label = cur_node.value - 1
        if destination_label < min_label:
            destination_label = max_label
        picked_up_values = {node.value for node in picked_up}
        while destination_label in picked_up_values:
            destination_label -= 1
            if destination_label < min_label:
                destination_label = max_label
        node_map[destination_label].insert_after(picked_up)
        cur_node = cur_node.next

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
    first_label = init_labels[0]
    first_node = Node(first_label)
    prev_node = first_node
    node_map = {first_label: first_node}
    for value in init_labels[1:]:
        node = Node(value)
        node_map[value] = node
        prev_node.next = node
        prev_node = node
    for value in range(max(init_labels) + 1, max_label + 1):
        node = Node(value)
        node_map[value] = node
        prev_node.next = node
        prev_node = node
    prev_node.next = first_node
    return first_node, node_map

def count_nodes(first_node):
    num_nodes = 1
    cur_node = first_node.next
    while cur_node.value != first_node.value:
        num_nodes += 1
        cur_node = cur_node.next
    return num_nodes


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

if __name__ == '__main__':
    main()
