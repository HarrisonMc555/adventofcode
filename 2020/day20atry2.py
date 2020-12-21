#!/usr/bin/env python3

import re
import math

INPUT_FILE = 'input20.txt'
# INPUT_FILE = 'example20.txt'

def main():
    text = get_text(INPUT_FILE)
    tiles = parse_text(text)
    # for tile_id, tile in tiles.items():
    #     print(f'Tile #{tile_id}')
    #     for row in tile:
    #         print(''.join(row))
    #     print()

    grid = assemble_image(tiles)
    # print(grid)
    # for row in grid:
    #     print(' '.join(str(tile_id) for tile_id in row))
    corners = [grid[0][0], grid[0][-1], grid[-1][0], grid[-1][-1]]
    # print(corners)
    print(product(corners))

def assemble_image(tiles):
    num_tiles = len(tiles)
    # for num_rows in divisors(num_tiles):
    # for num_rows in [3]:
    for num_rows in [int(math.sqrt(num_tiles))] + divisors(num_tiles):
        if num_tiles % num_rows != 0:
            continue
        num_columns = num_tiles // num_rows
        grid = [[None] * num_columns] * num_rows
        remaining = set(tiles.keys())
        result = assemble_image_helper(tiles, grid, remaining, 0, 0)
        if result:
            return [[tile_id for tile_id, _ in row] for row in result]
            # return result
    raise Exception('No image found')

DEBUG = False
def assemble_image_helper(tiles, grid, remaining, row_index, column_index):
    global DEBUG
    if row_index == 0 and column_index == 0:
        DEBUG = False
    if not remaining:
        return grid
    next_column_index = column_index + 1
    next_row_index = row_index
    if next_column_index == len(grid[0]):
        next_column_index = 0
        next_row_index += 1
    offset = row_index * len(grid[0]) + column_index
    if DEBUG:
        print(f'({row_index}, {column_index}) -> ({next_row_index}, {next_column_index})')
    for tile_id in remaining:
        next_remaining = remaining.copy()
        next_remaining.remove(tile_id)
        original_tile = tiles[tile_id]
        for tile in tile_variants(original_tile):
            if row_index > 0:
                _, tile_up = grid[row_index - 1][column_index]
                if not tiles_match_down(tile_up, tile):
                    continue
            if column_index > 0:
                _, tile_left = grid[row_index][column_index - 1]
                if not tiles_match_right(tile_left, tile):
                    continue
            grid[row_index][column_index] = tile_id, tile
            if tile_id == 1951 and row_index == 0 and column_index == 0:
                DEBUG = True
            if DEBUG:
                print(f'{" " * offset}Adding {tile_id} to ({row_index}, {column_index})')
                for row in tile:
                    print(''.join(row))
            next_grid = copy_grid(grid)
            result = assemble_image_helper(tiles, next_grid, next_remaining, next_row_index,
                                           next_column_index)
            if result:
                return result
    return None

def tile_variants(tile):
    result = []
    for tile2 in [tile, flip_horizontal(tile)]:
        for tile3 in [tile2, flip_vertical(tile)]:
            for _ in range(4):
                result.append(tile3)
                tile3 = rotate(tile3)
    return result

def rotate(tile):
    tile_width = len(tile)
    return [[tile[-j-1][i] for j in range(tile_width)] for i in range(tile_width)]

def flip_horizontal(tile):
    return [list(reversed(row)) for row in tile]

def flip_vertical(tile):
    return list(reversed(tile))

def tiles_match_down(tile_up, tile_down):
    return tile_up[-1] == tile_down[0]

def tiles_match_right(tile_left, tile_right):
    return [row[-1] for row in tile_left] == [row[0] for row in tile_right]

def get_text(filename):
    with open(filename) as f:
        return f.read().strip()

def parse_text(text):
    tiles_texts = text.split('\n\n')
    return dict(parse_tile(t) for t in tiles_texts)

HEADER_RE = re.compile('Tile (\d+):')
def parse_tile(text):
    lines = text.split('\n')
    header = lines[0]
    tile_id = int(HEADER_RE.match(header).group(1))
    lines = lines[1:]
    grid = [list(row) for row in lines]
    return tile_id, grid

def divisors(num):
    return [x for x in range(1, num + 1) if num % x == 0]

def copy_grid(grid):
    return [row[:] for row in grid]

def product(nums):
    result = 1
    for num in nums:
        result *= num
    return result

if __name__ == '__main__':
    main()
