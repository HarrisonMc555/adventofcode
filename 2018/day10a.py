#!/usr/bin/env python3
#pylint: disable=invalid-name

import sys
import re

def solve(pos_vels):
    pos_vels = find_message_pos_vels(pos_vels)
    return pos_vels_to_message(pos_vels)

def pos_vels_to_message(pos_vels):
    positions = set(pos for pos, vel in pos_vels)
    x_positions = [x for x, y in positions]
    min_x, max_x = min(x_positions), max(x_positions)
    y_positions = [y for x, y in positions]
    min_y, max_y = min(y_positions), max(y_positions)
    return [[message_char((x, y) in positions)
             for x in range(min_x, max_x + 1)]
            for y in range(min_y, max_y + 1)]

def get_width_height(positions):
    x_positions = [x for x, x in positions]
    min_x, max_x = min(x_positions), max(x_positions)
    y_positions = [y for x, y in positions]
    min_y, max_y = min(y_positions), max(y_positions)
    width, height = max_x - min_x + 1, max_y - min_y + 1
    return width, height

POINT_CHAR = '#'
BLANK_CHAR = '.'
def message_char(point_at_position):
    return POINT_CHAR if point_at_position else BLANK_CHAR

def find_message_pos_vels(pos_vels):
    while not done(pos_vels):
        pos_vels = step_all(pos_vels)
    return pos_vels

def step_all(pos_vels):
    return [step(pos_vel) for pos_vel in pos_vels]

def step(pos_vel):
    pos, vel = pos_vel
    x, y = pos
    dx, dy = vel
    newpos = (x + dx, y + dy)
    return newpos, vel

MESSAGE_HEIGHT = 10
def done(pos_vels):
    positions = [pos for pos, vel in pos_vels]
    y_positions = [y for x, y in positions]
    min_y, max_y = min(y_positions), max(y_positions)
    height = max_y - min_y + 1
    return height == MESSAGE_HEIGHT

def message_to_string(message):
    return '\n'.join([''.join(c for c in row) for row in message])

PATTERN = re.compile(
    r'position=< *(-?\d+), *(-?\d+)> velocity=< *(-?\d+), *(-?\d+)>')
def parse_line(line):
    x, y, dx, dy = [int(group) for group in PATTERN.match(line).groups()]
    return (x, y), (dx, dy)

def get_input():
    return [parse_line(line.strip()) for line in sys.stdin.readlines()]

def main():
    pos_vels = get_input()
    grid = solve(pos_vels)
    print('\n'.join([''.join(c for c in row) for row in grid]))

if __name__ == '__main__':
    main()
