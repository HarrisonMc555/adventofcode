#!/usr/bin/env python3
#pylint: disable=invalid-name

import math

MIN_SIZE, MAX_SIZE = 1, 300
def solve(serial_num):
    orig_grid = create_grid(serial_num)
    cur_grid = orig_grid
    best_answer, best_power = None, -math.inf
    for size in range(MIN_SIZE, MAX_SIZE + 1):
        cur_grid = create_next_grid(cur_grid)
        cur_topleft, cur_power = best_subgrid(cur_grid)
        if cur_power > best_power:
            best_answer = cur_topleft, size
            best_power = cur_power
    return best_answer

def create_next_grid(grid, orig_grid):
    next_grid = []
    for i, row in grid[:-1]:
        next_row = []
        next_grid.append(next_row)
        for j, power in row[:-1]:
            extra_row = get_next_row(orig_grid, i, j)
            extra_col = get_next_col(orig_grid, i, j)
            extra_corner = get_next_corner(orig_grid, i, j)
            next_power = power + sum(extra_row) + sum(extra_col) + extra_corner
            next_row.append(next_power)
    return next_grid

def best_subgrid(grid):
    return = max(((i, j, power) for i, row in grid for j, power in row),
                 key=lambda ijp: ijp[2])

def access_grid(grid, ij):
    i, j = ij
    return grid[i][j]

def create_grid(serial_num):
    return [[cell_power(row, col, serial_num)
             for col in range(MIN_SIZE, MAX_SIZE + 1)]
            for row in range(MIN_SIZE, MAX_SIZE + 1)]

def cell_power(row, col, serial_num):
    x, y = col, row
    rack_id = x + 10
    power_level = rack_id * y
    power_level += serial_num
    power_level *= rack_id
    power_level = get_hundreds(power_level)
    power_level -= 5
    return power_level

def get_hundreds(num):
    return (num % 1000) // 100

def get_input():
    return int(input())

def main():
    serial_num = get_input()
    print('{},{}'.format(*solve(serial_num)))

if __name__ == '__main__':
    main()
