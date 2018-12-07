#!/usr/bin/env python3

def safe_value(node):
    if node is not None:
        return node.value
    return None

class Node:
    def __init__(self, value, lst, next_node=None, prev_node=None):
        self.value = value
        self.lst = lst
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
        if prev_node is not None:
            prev_node.next_node = next_node
        if next_node is not None:
            next_node.prev_node = prev_node
            return_node = next_node
        else:
            return_node = prev_node
        if self.lst.head == self:
            self.lst.head = return_node
        if self.lst.tail == self:
            self.lst.tail = return_node
        return return_node

    def push(self, next_value):
        node = Node(next_value, self.lst, prev_node=self,
                    next_node=self.next_node)
        if self.next_node is not None:
            self.next_node.prev_node = node
        self.next_node = node
        return node

class LinkedList:
    def __init__(self, iterable=None):
        self.head = None
        self.tail = None
        if iterable is not None:
            for value in iterable:
                self.push(value)

    def __iter__(self):
        cur_node = self.head
        while cur_node is not None:
            yield cur_node
            cur_node = cur_node.next_node

    def __len__(self):
        length = 0
        for _ in self:
            length += 1
        return length

    def __str__(self, sep=', '):
        return 'LinkedList([{}])'.format([repr(node) for node in self])

    def push(self, value):
        if self.head is not None:
            self.tail = self.tail.push(value)
            return
        node = Node(value, self)
        self.head = node
        self.tail = node

    def pop(self):
        if self.tail is None:
            return None
        old_tail_value = self.tail.value
        self.tail = self.tail.prev_node
        if self.tail is None:
            self.head = None
        return old_tail_value

    def to_string(self, sep=', '):
        return sep.join(str(node) for node in self)

def solve(polymer):
    return len(react(polymer))

def react(polymer):
    polymer = LinkedList(polymer)
    node = polymer.head
    while node is not None and node.next_node is not None:
        unit1, unit2 = node.value, node.next_node.value
        if is_opposite(unit1, unit2):
            node.next_node.pop()
            node = node.pop()
            if node is not None and node.prev_node is not None:
                node = node.prev_node
        else:
            node = node.next_node
    return polymer

def is_opposite(unit1, unit2):
    return unit1.lower() == unit2.lower() and unit1 != unit2

def get_input():
    return input()

def main():
    polymer = get_input()
    print(solve(polymer))

if __name__ == '__main__':
    main()
