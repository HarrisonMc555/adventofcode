#!/usr/bin/env python3
#pylint: disable=invalid-name

MIN_SIZE, MAX_SIZE = 1, 300
MIN_X, MAX_X = MIN_SIZE, MAX_SIZE
MIN_Y, MAX_Y = MIN_SIZE, MAX_SIZE
def solve(serial_num):
    grid = create_grid(serial_num)
    topleft, size, _power = max(range(MIN_SIZE, MAX_SIZE + 1),
                                key=lambda size: best_subgrid(grid, size))
    return (*topleft, size)

def best_subgrid(grid, size):
    print(size)
    toplefts = find_subgrid_toplefts(grid, size)
    best_topleft = max(toplefts,
                       key=lambda topleft: subgrid_power(grid, size, *topleft))
    return best_topleft, size, subgrid_power(grid, size, *best_topleft)

def create_grid(serial_num):
    return [[cell_power(row, col, serial_num)
             for col in range(MIN_X, MAX_X + 1)]
            for row in range(MIN_Y, MAX_Y + 1)]

def cell_power(row, col, serial_num):
    x, y = col, row
    rack_id = x + 10
    power_level = rack_id * y
    power_level += serial_num
    power_level *= rack_id
    power_level = get_hundreds(power_level)
    power_level -= 5
    return power_level

def subgrid_power(grid, size, row, col):
    return total_power(get_subgrid(grid, size, row, col))

def get_subgrid(grid, size, row, col):
    return [[grid[col + dcol - 1][row + drow - 1]
             for dcol in range(size)]
            for drow in range(size)]

def total_power(subgrid):
    return sum(sum(row) for row in subgrid)

def find_subgrid_toplefts(grid, size):
    return [(row, col) for row in range(len(grid) - size + 1)
            for col in range(len(grid[row]) - size + 1)]

def get_hundreds(num):
    return (num % 1000) // 100

def get_input():
    return int(input())

def main():
    serial_num = get_input()
    print('{},{}'.format(*solve(serial_num)))

if __name__ == '__main__':
    main()
