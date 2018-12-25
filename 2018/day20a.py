#!/usr/bin/env python3
#pylint: disable=invalid-name

from collections import defaultdict

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

def build_graph(regex):
    before_group_stack = []
    in_group_stack = []
    start_pos = (0, 0)
    cur_positions = set([start_pos])
    group_level = 0
    connections = defaultdict(lambda: [None]*4)
    for char in regex:
        if is_begin_group(char):
            group_level += 1
            before_group_stack.append(cur_positions)
            in_group_stack.append(set())
        elif is_divide_group(char):
            in_group_stack[-1].update(cur_positions)
            cur_positions = before_group_stack[-1]
        elif is_end_group(char):
            assert group_level > 0
            group_level -= 1
            in_group_stack[-1].update(cur_positions)
            cur_positions = in_group_stack.pop()
            before_group_stack.pop()
        else:
            direction = get_direction(char)
            cur_positions = add_direction_to_all(direction, cur_positions,
                                                 connections)
    assert group_level == 0
    return connections

def find_longest_path_length(connections):
    import math
    start_pos = (0, 0)
    positions = [start_pos]
    distances = defaultdict(lambda: math.inf)
    distances[start_pos] = 0
    while positions:
        position = positions.pop()
        assert position in distances
        distance = distances[position]
        next_distance = distance + 1
        for next_position in connections[position]:
            if next_position is not None and \
               next_distance < distances[next_position]:
                distances[next_position] = next_distance
                positions.append(next_position)
    return max(distances[p] for p in connections)

NORTH = 0
EAST = 1
SOUTH = 2
WEST = 3

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
    end_pos = (x2, y2)
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
