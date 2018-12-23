#!/usr/bin/env python3
#pylint: disable=invalid-name

from collections import defaultdict
# from enum import Enum, auto

def main():
    regex = get_input()
    print(solve(regex))

def get_input():
    full_regex = input()
    assert full_regex[0] == '^' and full_regex[-1] == '$'
    regex = full_regex[1:-1]
    return regex

def solve(regex):
    connections = build_graph(regex)
    return find_longest_path_length(connections)

# def find_longest_path_length_from_regex(regex):
#     pass

#pylint: disable=too-few-public-methods
class Node:
    def __init__(self):
        self.north = None
        self.east = None
        self.south = None
        self.west = None

def build_graph(regex):
    before_group_stack = []
    in_group_stack = []
    start_pos = (0, 0)
    cur_positions = [start_pos]
    group_level = 0
    # connections = defaultdict(set)
    connections = defaultdict(lambda: [None]*4)
    for char in regex:
        # print('Processing: \'{}\''.format(char))
        # get_prefix = lambda: '\t' * (group_level + 1)
        if is_begin_group(char):
            group_level += 1
            # print(get_prefix(), 'beginning group')
            before_group_stack.append(cur_positions)
            in_group_stack.append([])
        elif is_divide_group(char):
            # print(get_prefix(), 'dividing group')
            in_group_stack[-1].extend(cur_positions)
            cur_positions = before_group_stack[-1]
        elif is_end_group(char):
            # print(get_prefix(), 'ending group')
            assert group_level > 0
            group_level -= 1
            in_group_stack[-1].extend(cur_positions)
            cur_positions = in_group_stack.pop()
            before_group_stack.pop()
        else:
            direction = get_direction(char)
            cur_positions = add_direction_to_all(direction, cur_positions,
                                                 connections)
    assert group_level == 0
    # for k, v in connections.items():
    #     print('{}: {}'.format(k, v))
    return connections

def find_longest_path_length(connections):
    import math
    start_pos = (0, 0)
    positions = [start_pos]
    distances = defaultdict(lambda: math.inf)
    distances[start_pos] = 0
    while positions:
        position = positions.pop()
        # assert position in distances
        distance = distances[position]
        next_distance = distance + 1
        for next_position in connections[position]:
            # if next_distance < distances[next_position]:
            if next_position is not None and \
               next_distance < distances[next_position]:
                distances[next_position] = next_distance
                positions.append(next_position)
    return max(distances[p] for p in connections)

# class Direction(Enum):
#     NORTH = auto()
#     EAST = auto()
#     SOUTH = auto()
#     WEST = auto()

NORTH = 0
EAST = 1
SOUTH = 2
WEST = 3

# CHAR_TO_DIRECTION = {
#     'N': Direction.NORTH,
#     'E': Direction.EAST,
#     'S': Direction.SOUTH,
#     'W': Direction.WEST,
# }
CHAR_TO_DIRECTION = {
    'N': NORTH,
    'E': EAST,
    'S': SOUTH,
    'W': WEST,
}

def get_direction(char):
    assert is_direction_char(char)
    return CHAR_TO_DIRECTION[char]

def add_direction_to_all(direction, cur_positions, connections):
    return [add_direction_to_one(direction, position, connections)
            for position in cur_positions]
    # for position in cur_positions:
    #     add_direction_to_one(direction, position, connections)

# DXDYS = {
#     Direction.NORTH: (0, -1),
#     Direction.EAST: (1, 0),
#     Direction.SOUTH: (0, 1),
#     Direction.WEST: (-1, 0),
# }

DXDYS = [
    (0, -1), # NORTH (0)
    (1, 0),  # WEST  (1)
    (0, 1),  # SOUTH (2)
    (-1, 0), # EAST  (3)
]

def add_direction_to_one(direction, start_pos, connections):
    x1, y1 = start_pos
    dx, dy = DXDYS[direction]
    x2, y2 = x1 + dx, y1 + dy
    # if direction == Direction.NORTH:
    #     x2, y2 = x1, y1 - 1
    # elif direction == Direction.EAST:
    #     x2, y2 = x1 + 1, y1
    # elif direction == Direction.SOUTH:
    #     x2, y2 = x1, y1 + 1
    # elif direction == Direction.WEST:
    #     x2, y2 = x1 - 1, y1
    # else:
    #     raise Exception('Invalid direction')
    end_pos = (x2, y2)
    # connections[start_pos].append(end_pos)
    # connections[end_pos].append(start_pos)
    # if end_pos in connections[start_pos]:
    #     print('{} already connected to {}'.format(start_pos, end_pos))
    # if start_pos in connections[end_pos]:
    #     print('{} already connected to {}'.format(end_pos, start_pos))
    # connections[start_pos].add(end_pos)
    # connections[end_pos].add(start_pos)
    connections[start_pos][direction] = end_pos
    connections[end_pos][opposite_direction(direction)] = start_pos
    return end_pos

def opposite_direction(direction):
    return direction + 2 if direction < 2 else direction - 2

def is_direction_char(char):
    return char in CHAR_TO_DIRECTION

def is_begin_group(char):
    return char == '('

def is_divide_group(char):
    return char == '|'

def is_end_group(char):
    return char == ')'

if __name__ == '__main__':
    main()
