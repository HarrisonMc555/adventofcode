#!/usr/bin/env python3
#pylint: disable=invalid-name

def solve(serial_num, max_size):
    orig_grid = create_grid(serial_num, max_size)
    # print(grid_string(orig_grid))
    # print()
    cur_grid = orig_grid
    best_topleft, best_power = best_subgrid(orig_grid)
    best_answer = best_topleft, 1
    for size in range(2, max_size + 1):
        print('size:', size)
        cur_grid = create_next_grid(cur_grid, orig_grid)
        # print('cur_grid:\n', grid_string(cur_grid))
        # print('done')
        # input()
        cur_topleft, cur_power = best_subgrid(cur_grid)
        # print('cur_topleft: {}, cur_power: {}'.format(
        #     cur_topleft, cur_power))
        if cur_power > best_power:
            best_answer = cur_topleft, size
            best_power = cur_power
    topleft, size = best_answer
    row, col = topleft
    x = col + 1
    y = row + 1
    return x, y, size

def grid_string(grid):
    return '\n'.join(''.join('{: 3}'.format(cell) for cell in row)
                     for row in grid)

def create_next_grid(grid, orig_grid):
    size = len(orig_grid) - len(grid) + 1
    outer_size = size + 1
    # print('outer_size:', outer_size)
    next_grid = []
    for i, row in enumerate(grid[:-1]):
        next_row = []
        for j, power in enumerate(row[:-1]):
            outer_row = get_outer_row(orig_grid, i, j, outer_size)
            outer_col = get_outer_col(orig_grid, i, j, outer_size)
            outer_corner = get_outer_corner(orig_grid, i, j, outer_size)
            # we're double counting the corner, so remove it
            outer_power = power + sum(outer_row) + sum(outer_col) - outer_corner
            # print('at ({}, {}), the cell is {}'.format(i, j, power))
            # print('\touter_row:', outer_row)
            # print('\touter_col:', outer_col)
            # print('\touter_corner:', outer_corner)
            # print('\touter_power:', outer_power)
            next_row.append(outer_power)
        next_grid.append(next_row)
    return next_grid

def get_outer_row(grid, i, j, size):
    return grid[i + size - 1][j:j + size]

def get_outer_col(grid, i, j, size):
    return [row[j + size - 1] for row in grid[i:i + size]]

def get_outer_corner(grid, i, j, size):
    return grid[i + size - 1][j + size - 1]

def best_subgrid(grid):
    i, j, power = max(((i, j, power) for i, row in enumerate(grid)
                       for j, power in enumerate(row)),
                      key=lambda ijp: ijp[2])
    return (i, j), power

def access_grid(grid, ij):
    i, j = ij
    return grid[i][j]

def create_grid(serial_num, max_size):
    return [[cell_power(row, col, serial_num)
             for col in range(1, max_size + 1)]
            for row in range(1, max_size + 1)]

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
    max_size = 300
    serial_num = get_input()
    x, y, size = solve(serial_num, max_size)
    print('{},{},{}'.format(x, y, size))

if __name__ == '__main__':
    main()
