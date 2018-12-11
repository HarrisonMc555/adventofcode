#!/usr/bin/env python3
#pylint: disable=invalid-name

MIN_X, MAX_X = 1, 300
MIN_Y, MAX_Y = 1, 300
def solve(serial_num):
    grid = create_grid(serial_num)
    toplefts = find_subgrid_toplefts(grid)
    best_topleft = max(toplefts,
                       key=lambda topleft: subgrid_power(grid, *topleft))
    return best_topleft

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

def subgrid_power(grid, row, col):
    return total_power(get_subgrid(grid, row, col))

def get_subgrid(grid, row, col):
    return [[grid[col + dcol - 1][row + drow - 1]
             for dcol in range(SUBGRID_WIDTH)]
            for drow in range(SUBGRID_HEIGHT)]

def total_power(subgrid):
    return sum(sum(row) for row in subgrid)

SUBGRID_WIDTH, SUBGRID_HEIGHT = 3, 3
def find_subgrid_toplefts(grid):
    return [(row, col) for row in range(len(grid) - SUBGRID_HEIGHT + 1)
            for col in range(len(grid[row]) - SUBGRID_WIDTH + 1)]

def get_hundreds(num):
    return (num % 1000) // 100

def get_input():
    return int(input())

def main():
    serial_num = get_input()
    print('{},{}'.format(*solve(serial_num)))

if __name__ == '__main__':
    main()
