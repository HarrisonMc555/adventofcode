#!/usr/bin/env python3

from enum import Enum, auto

INPUT_FILE = 'input11.txt'
# INPUT_FILE = 'example.txt'

def main():
    lines = get_lines(INPUT_FILE)
    grid = parse_lines(lines)
    # for _ in range(7):
    #     print_grid(grid)
    #     print()
    #     grid = step(grid)
    final_grid = step_until_stable(grid)
    print(count_occupied(final_grid))

def get_lines(filename):
    with open(filename) as f:
        return [line.strip() for line in f.readlines() if line]

def parse_lines(lines):
    return [parse_line(line) for line in lines]

def parse_line(line):
    return [Tile.from_char(c) for c in line]

class Tile(Enum):
    FLOOR = auto()
    EMPTY = auto()
    OCCUPIED = auto()

    def __str__(self):
        return TILE_TO_CHAR[self]

    def from_char(c):
        return CHAR_TO_TILE[c]

TILE_TO_CHAR = {
    Tile.FLOOR: '.',
    Tile.EMPTY: 'L',
    Tile.OCCUPIED: '#',
}

CHAR_TO_TILE = {
    '.': Tile.FLOOR,
    'L': Tile.EMPTY,
    '#': Tile.OCCUPIED,
}

def step(grid):
    next_grid = []
    num_rows = len(grid)
    num_cols = len(grid[0])
    for i in range(num_rows):
        next_row = []
        for j in range(num_cols):
            next_row.append(step_tile(grid, i, j))
        next_grid.append(next_row)
    return next_grid

def step_tile(grid, i, j):
    cur_tile = grid[i][j]
    num_occupied_neighbors = count_occupied_neighbors(grid, i, j)
    if cur_tile == Tile.EMPTY and num_occupied_neighbors == 0:
        return Tile.OCCUPIED
    elif cur_tile == Tile.OCCUPIED and num_occupied_neighbors >= 5:
        return Tile.EMPTY
    else:
        return cur_tile

def count_occupied_neighbors(grid, i, j):
    indices = get_neighboring_indices(grid, i, j)
    neighbors = [grid[i2][j2] for i2, j2 in indices]
    return neighbors.count(Tile.OCCUPIED)

def get_neighboring_indices(grid, i, j):
    indices = []
    for di, dj in OFFSETS:
        i2, j2 = i + di, j + dj
        while in_range_row(grid, i2) and in_range_col(grid, j2):
            if grid[i2][j2] != Tile.FLOOR:
                indices.append((i2, j2))
                break
            i2 += di
            j2 += dj
    return indices

def in_range_row(grid, row):
    return 0 <= row < len(grid)

def in_range_col(grid, col):
    return 0 <= col < len(grid[0])

OFFSETS = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
]
    
def step_until_stable(grid):
    prev_grid = grid
    next_grid = step(grid)
    while prev_grid != next_grid:
        prev_grid = next_grid
        next_grid = step(next_grid)
    return prev_grid

def count_occupied(grid):
    return sum(row.count(Tile.OCCUPIED) for row in grid)

def print_grid(grid):
    for row in grid:
        print(''.join(str(t) for t in row))

if __name__ == '__main__':
    main()
