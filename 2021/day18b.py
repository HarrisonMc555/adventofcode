#!/usr/bin/env python3

INPUT_FILE = 'input18.txt'

import unittest
from typing import Union
from enum import Enum, auto
from dataclasses import dataclass

DEBUG = False
# DEBUG = True
def debug_print(*args, **kwargs):
    if DEBUG:
        print(*args, **kwargs)

def main():
    if 'INPUT_FILE' not in globals():
        unittest.main()
        return
    lines = get_lines(INPUT_FILE)
    print(run(lines))

def run(lines):
    lists = [eval(line) for line in lines]
    pairs = get_pairs(lists)
    return max(add_lists_magnitude(left, right) for left, right in pairs)

def add_lists_magnitude(left_list, right_list):
    left = list_to_tree(left_list)
    right = list_to_tree(right_list)
    return magnitude(add(left, right))

def get_pairs(lst):
    for i in range(len(lst) - 1):
        for j in range(i + 1, len(lst)):
            yield lst[i], lst[j]
            yield lst[j], lst[i]

def magnitude(tree):
    if isinstance(tree, Node):
        return 3 * magnitude(tree.left) + 2 * magnitude(tree.right)
    else:
        return tree

def add_trees(trees):
    tree = trees[0]
    for new_tree in trees[1:]:
        tree = add(tree, new_tree)
    return tree

def add(left, right):
    tree = Node(left, right)
    keep_going = True
    while keep_going:
        tree, keep_going = run_step(tree)
    return tree

def run_step(tree):
    tree, result = try_explode(tree)
    if result:
        return tree, result
    tree, result = try_split(tree)
    if result:
        return tree, result
    return tree, False

def try_explode(tree):
    zipper = Zipper(tree, When.Before)
    try:
        while zipper.depth() < 4 or zipper.is_leaf():
            zipper.forward()
        assert zipper.depth() == 4
        assert not isinstance(zipper.cur.left, Node)
        assert not isinstance(zipper.cur.right, Node)
        left, right = zipper.cur.left, zipper.cur.right
        zipper.replace(0)
        try:
            # debug_print('**** Going to prev')
            zipper.prev_leaf()
            # debug_print('**** Replacing left')
            zipper.replace(zipper.cur + left)
        except ZipperException:
            # debug_print('**** Could not find previous leaf')
            pass
        # debug_print('**** Going back to orig')
        zipper.next_leaf()
        try:
            # debug_print('**** Going to next')
            zipper.next_leaf()
            # debug_print('**** Replacing right')
            zipper.replace(zipper.cur + right)
            # debug_print('**** Done replacing right')
        except ZipperException:
            # debug_print('**** Could not find next leaf')
            pass
        return zipper.root, True
    except ZipperException:
        return zipper.root, False

def try_split(tree):
    zipper = Zipper(tree, When.Before)
    try:
        zipper.next_leaf()
        while zipper.cur < 10:
            zipper.next_leaf()
        left = zipper.cur // 2
        right = zipper.cur - left
        zipper.replace(Node(left, right))
        return zipper.root, True
    except ZipperException:
        return zipper.root, False

class Dir(Enum):
    Left = auto()
    Right = auto()

def list_to_tree(value):
    if isinstance(value, list):
        if len(value) != 2:
            raise Exception(f'List to tree was not length 2: {value}')
        return Node(list_to_tree(value[0]), list_to_tree(value[1]))
    else:
        return value

def tree_to_list(tree):
    if isinstance(tree, Node):
        return [tree_to_list(tree.left), tree_to_list(tree.right)]
    else:
        return tree

def traverse(tree):
    if isinstance(tree, Node):
        yield from traverse(tree.left)
        yield from traverse(tree.right)
    else:
        yield tree

@dataclass
class Node:
    left: Union['Node', int]
    right: Union['Node', int]

    def __str__(self):
        return f'[{self.left},{self.right}]'

    def __repr__(self):
        return str(self)

class ZipperException(Exception):
    pass

class When(Enum):
    Before = auto()
    Middle = auto()
    After = auto()

@dataclass
class Zipper:
    root: Union[Node, int]
    cur: Union[Node, int]
    path: [Union[Node, int]]
    dirs: [Dir]
    when: 'When'

    def __init__(self, root, when):
        self.root = root
        self.cur = root
        self.path = []
        self.dirs = []
        self.when = when

    def depth(self):
        return len(self.path)

    def is_node(self):
        return isinstance(self.cur, Node)

    def is_leaf(self):
        return not self.is_node()

    def replace(self, sub_tree):
        if self.dirs:
            if self.dirs[-1] == Dir.Left:
                self.path[-1].left = sub_tree
            else:
                self.path[-1].right = sub_tree
        else:
            self.root = sub_tree
        self.cur = sub_tree

    def address(self):
        return (tuple(self.dirs), self.when)

    def left(self, when):
        # debug_print(f'Before left: {self}')
        if self.is_leaf():
            raise ZipperException()
        self.path.append(self.cur)
        self.dirs.append(Dir.Left)
        self.when = when
        self.cur = self.cur.left
        # debug_print(f'After left: {self}')

    def right(self, when):
        # debug_print(f'Before right: {self}')
        if self.is_leaf():
            raise ZipperException()
        self.path.append(self.cur)
        self.dirs.append(Dir.Right)
        self.when = when
        self.cur = self.cur.right

    def up(self, when):
        # debug_print(f'Before up: {self}')
        if not self.path:
            raise ZipperException()
        self.cur = self.path.pop()
        prev_dir = self.dirs.pop()
        self.when = when
        # debug_print(f'After up: {self}')

    def forward(self):
        if self.is_leaf():
            if not self.dirs:
                raise ZipperException()
            if self.dirs[-1] == Dir.Left:
                when = When.Middle
            else:
                when = When.After
            self.up(when)
        else:
            if self.when == When.Before:
                self.left(When.Before)
            elif self.when == When.Middle:
                self.right(When.Before)
            elif self.when == When.After:
                if not self.dirs:
                    raise ZipperException()
                if self.dirs[-1] == Dir.Left:
                    when = When.Middle
                else:
                    when = When.After
                self.up(when)
            else:
                raise Exception('Non-exhaustive when case')

    def next_leaf(self):
        self.forward()
        while not self.is_leaf():
            self.forward()

    def backward(self):
        if self.is_leaf():
            if not self.dirs:
                raise ZipperException()
            if self.dirs[-1] == Dir.Right:
                when = When.Middle
            else:
                when = When.Before
            self.up(when)
        else:
            if self.when == When.Before:
                if not self.dirs:
                    raise ZipperException()
                if self.dirs[-1] == Dir.Right:
                    when = When.Middle
                else:
                    when = When.Before
                self.up(when)
            elif self.when == When.Middle:
                self.left(When.After)
            elif self.when == When.After:
                self.right(When.After)
            else:
                raise Exception('Non-exhaustive when case')

    def prev_leaf(self):
        self.backward()
        while not self.is_leaf():
            self.backward()

BEFORE_TO_AFTER_EXPLODE = [
    ([[[[[9,8],1],2],3],4], [[[[0,9],2],3],4]),
    ([7,[6,[5,[4,[3,2]]]]], [7,[6,[5,[7,0]]]]),
    ([[6,[5,[4,[3,2]]]],1], [[6,[5,[7,0]]],3]),
    ([[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]], [[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]),
    ([[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]], [[3,[2,[8,0]]],[9,[5,[7,0]]]]),
]
BEFORE_TO_AFTER_SPLIT = [
    (10, [5,5]),
    (11, [5,6]),
    (12, [6,6]),
]
ROUND_TRIP_EXAMPLES = [
    42,
    [9, 8],
    [[4, 7], [5, 6]],
    [7, [9, 1]],
]
TREE_TO_FLAT = [
    (42, [42]),
    ([9, 8], [9, 8]),
    ([[4, 7], [5, 6]], [4, 7, 5, 6]),
    ([7, [9, 1]], [7, 9, 1]),
    ([[[87, 3], [4, 27]], [9, 1]], [87, 3, 4, 27, 9, 1]),
]
EXAMPLE_LEFT = [[[[4,3],4],4],[7,[[8,4],9]]]
EXAMPLE_RIGHT = [1,1]
EXAMPLE_STEPS = [
    [[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]],
    [[[[0,7],4],[7,[[8,4],9]]],[1,1]],
    [[[[0,7],4],[15,[0,13]]],[1,1]],
    [[[[0,7],4],[[7,8],[0,13]]],[1,1]],
    [[[[0,7],4],[[7,8],[0,[6,7]]]],[1,1]],
    [[[[0,7],4],[[7,8],[6,0]]],[8,1]],
]
NUMS_TO_SUM = [
    ([[1,1], [2,2], [3,3], [4,4]],
     [[[[1,1],[2,2]],[3,3]],[4,4]]),
    ([[1,1], [2,2], [3,3], [4,4], [5,5]],
     [[[[3,0],[5,3]],[4,4]],[5,5]]),
    ([[1,1], [2,2], [3,3], [4,4], [5,5], [6,6]],
     [[[[5,0],[7,4]],[5,5]],[6,6]]),
]
MAGNITUDE_INPUT = [
    [[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]],
    [[[5,[2,8]],4],[5,[[9,9],0]]],
    [6,[[[6,2],[5,6]],[[7,6],[4,7]]]],
    [[[6,[0,7]],[0,9]],[4,[9,[9,0]]]],
    [[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]],
    [[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]],
    [[[[5,4],[7,7]],8],[[8,3],8]],
    [[9,3],[[9,9],[6,[4,9]]]],
    [[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]],
    [[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]],
]
MAGNITUDE_OUTPUT = 4140
PART_2_EXAMPLE_TEXT = '''
[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
[[[5,[2,8]],4],[5,[[9,9],0]]]
[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
[[[[5,4],[7,7]],8],[[8,3],8]]
[[9,3],[[9,9],[6,[4,9]]]]
[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]'''
PART_2_MAGNITUDE = 3993
PART_2_LEFT = [[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
PART_2_RIGHT = [[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
PART_2_SUM = [[[[7,8],[6,6]],[[6,0],[7,7]]],[[[7,8],[8,8]],[[7,9],[0,6]]]]
class Test(unittest.TestCase):
    def test_list_to_tree(self):
        for orig in ROUND_TRIP_EXAMPLES:
            tree = list_to_tree(orig)
            round_trip = tree_to_list(tree)
            self.assertEqual(round_trip, orig)
            self.assertEqual(str(tree), Test.list_str(orig))

    def test_traverse(self):
        for tree_list, expected in TREE_TO_FLAT:
            tree = list_to_tree(tree_list)
            actual = list(traverse(tree))
            self.assertEqual(actual, expected, f'orig: {tree_list}')

    def test_zipper_forward(self):
        for tree_list, expected in TREE_TO_FLAT:
            tree = list_to_tree(tree_list)
            zipper = Zipper(tree, When.Before)
            actual = []
            if zipper.is_leaf():
                actual.append(zipper.cur)
            prev_addresses = set([zipper.address()])
            while True:
                try:
                    zipper.forward()
                    address = zipper.address()
                    if address in prev_addresses:
                        raise Exception(f'Infinite loop: {zipper}')
                    prev_addresses.add(address)
                    if zipper.is_leaf():
                        actual.append(zipper.cur)
                except ZipperException:
                    break
            self.assertEqual(actual, expected)

    def test_zipper_backward(self):
        for tree_list, expected in TREE_TO_FLAT:
            tree = list_to_tree(tree_list)
            zipper = Zipper(tree, When.After)
            actual = []
            if zipper.is_leaf():
                actual.append(zipper.cur)
            prev_addresses = {zipper.address()}
            while True:
                try:
                    zipper.backward()
                    address = zipper.address()
                    if address in prev_addresses:
                        raise Exception(f'Infinite loop: {zipper}')
                    prev_addresses.add(address)
                    if zipper.is_leaf():
                        actual.append(zipper.cur)
                except ZipperException:
                    break
            self.assertEqual(actual, list(reversed(expected)), f'orig: {tree_list}')

    def test_explode(self):
        for before, after in BEFORE_TO_AFTER_EXPLODE:
            tree = list_to_tree(before)
            tree, result = try_explode(tree)
            self.assertEqual(tree_to_list(tree), after, f'Original: {before}')
            self.assertTrue(result)

    # def test_split(self):
    #     for before, after in BEFORE_TO_AFTER_SPLIT:
    #         tree = list_to_tree(before)
    #         tree, result = try_split(tree)
    #         self.assertEqual(tree_to_list(tree), after, f'Original: {before}')
    #         self.assertTrue(result)

    def test_example(self):
        left = list_to_tree(EXAMPLE_LEFT)
        right = list_to_tree(EXAMPLE_RIGHT)
        cur = Node(left, right)
        for step, expected in enumerate(EXAMPLE_STEPS):
            self.assertEqual(tree_to_list(cur), expected, f'Step {step}: {expected}')
            cur, result = run_step(cur)

    def test_small_examples(self):
        for nums, expected_list in NUMS_TO_SUM:
            trees = [list_to_tree(num) for num in nums]
            actual = add_trees(trees)
            actual_list = tree_to_list(actual)
            self.assertEqual(actual_list, expected_list)

    def test_magnitude(self):
        trees = [list_to_tree(num) for num in MAGNITUDE_INPUT]
        actual = magnitude(add_trees(trees))
        self.assertEqual(actual, MAGNITUDE_OUTPUT)

    def test_get_pairs(self):
        letters = 'abcd'
        actual = list(get_pairs(letters))
        expected = [
            ('a', 'b'),
            ('b', 'a'),
            ('a', 'c'),
            ('c', 'a'),
            ('a', 'd'),
            ('d', 'a'),
            ('b', 'c'),
            ('c', 'b'),
            ('b', 'd'),
            ('d', 'b'),
            ('c', 'd'),
            ('d', 'c'),
        ]
        self.assertEqual(actual, expected)

    def test_part_2_sum(self):
        left = list_to_tree(PART_2_LEFT)
        right = list_to_tree(PART_2_RIGHT)
        result = add(left, right)
        self.assertEqual(tree_to_list(result), PART_2_SUM)
        self.assertEqual(magnitude(result), PART_2_MAGNITUDE)

    def test_part_2_lines(self):
        lines = PART_2_EXAMPLE_TEXT.strip().split('\n')
        result = run(lines)
        self.assertEqual(result, PART_2_MAGNITUDE)

    @staticmethod
    def list_str(lst):
        if isinstance(lst, list):
            return '[' + ','.join(Test.list_str(v) for v in lst) + ']'
        else:
            return str(lst)

def get_lines(filename):
    with open(filename) as f:
        return [line.strip() for line in f.readlines() if line]

if __name__ == '__main__':
    main()
