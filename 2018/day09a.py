#!/usr/bin/env python3

import re
from collections import Counter

class Node:
    def __init__(self, value, lst, next_node=None, prev_node=None):
        self.value = value
        self.lst = lst
        if next_node is None or prev_node is None:
            assert next_node is None and prev_node is None
        if next_node is None:
            next_node = self
            prev_node = self
        self.next_node = next_node
        self.prev_node = prev_node

    def __str__(self):
        return str(self.value)

    def __repr__(self):
        return 'Node({}, {}, {})'.format(self.value, self.next_node,
                                         self.prev_node)

    def pop(self):
        """Deletes the node and returns the one that takes its place

        The former previous and next nodes are linked together, the current node
        is deleted, and the former next node is returned.
        """
        prev_node, next_node = self.prev_node, self.next_node
        prev_node.next_node = next_node
        next_node.prev_node = prev_node
        return_node = next_node
        if self.lst.head == self:
            self.lst.head = return_node
        return return_node

    def insert_after(self, next_value):
        node = Node(next_value, self.lst, prev_node=self,
                    next_node=self.next_node)
        self.next_node.prev_node = node
        self.next_node = node
        return node

class CircularLinkedList:
    def __init__(self, iterable=None):
        self.head = None
        if iterable is not None:
            for value in iterable:
                self.push(value)

    def __iter__(self):
        if self.head is None:
            return
        yield self.head
        cur_node = self.head.next_node
        while cur_node is not self.head:
            yield cur_node
            cur_node = cur_node.next_node

    def __len__(self):
        length = 0
        for _ in self:
            length += 1
        return length

    def __str__(self, sep=', '):
        return 'LinkedList({})'.format([repr(node) for node in self])

    def push(self, value):
        if self.head is not None:
            self.head.prev_node.insert_after(value)
            return
        node = Node(value, self)
        self.head = node
        self.head.next_node = self.head
        self.head.prev_node = self.head

    def to_string(self, sep=', '):
        return sep.join(str(node) for node in self)

def turn(player, marble, cur_marble, scores):
    if is_scoring_turn(marble):
        return scoring_turn(player, marble, cur_marble, scores)
    left = cur_marble.next_node
    cur_marble = left.insert_after(marble)
    return cur_marble, scores

SCORE_TURN_NUM_COUNTER_CLOCKWISE = 7
def scoring_turn(player, marble, cur_marble, scores):
    scores[player] += marble
    for _ in range(SCORE_TURN_NUM_COUNTER_CLOCKWISE):
        cur_marble = cur_marble.prev_node
    scores[player] += cur_marble.value
    cur_marble = cur_marble.pop()
    return cur_marble, scores

SCORE_TURN_NUMBER = 23
def is_scoring_turn(marble):
    return marble % SCORE_TURN_NUMBER == 0

def next_player(player, num_players):
    return (player % num_players) + 1

def play_game(num_players, last_marble):
    scores = Counter()
    marbles = CircularLinkedList()
    marbles.push(0)
    cur_marble = marbles.head
    player = 1
    for marble in range(1, last_marble + 1):
        cur_marble, scores = turn(player, marble, cur_marble, scores)
        player = next_player(player, num_players)
    return scores

def round_string(marbles, player, cur_marble):
    strings = []
    strings.append('[{}] '.format(player))
    for marble in marbles:
        if marble is cur_marble:
            strings.append('({}) '.format(marble.value))
        else:
            strings.append(' {}  '.format(marble.value))
    return ''.join(strings)

def solve(num_players, last_marble):
    scores = play_game(num_players, last_marble)
    return max(scores.values())

PATTERN = re.compile(r'(\d+) players; last marble is worth (\d+) points')
def get_input():
    num_players, last_marble = PATTERN.match(input()).groups()
    num_players, last_marble = int(num_players), int(last_marble)
    return num_players, last_marble

def main():
    num_players, last_marble = get_input()
    print(solve(num_players, last_marble))

if __name__ == '__main__':
    main()
