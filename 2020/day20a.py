#!/usr/bin/env python3

import re
from enum import Enum, auto

# INPUT_FILE = 'input20.txt'
INPUT_FILE = 'example20.txt'

def main():
    # unittest.main()
    text = get_text(INPUT_FILE)
    tiles = parse_text(text)
    # for tile_id, grid in tiles.items():
    #     print(f'Tile {tile_id}:')
    #     for row in grid:
    #         print(''.join(row))
    #     print()

    tile_id = next(iter(tiles))
    print(f'Tile {tile_id}')
    tile = tiles[tile_id]
    for row in tile:
        print(''.join(row))
    print()
    for direction in Direction:
        print(f'\tDirection: {direction:15} => {"".join(get_edge(tile, direction))}')
    print()

    for tile_id in tiles:
        matching = find_matching_directions(tile_id, tiles)
        for direction, other_direction, other_tile_id in matching:
            print(f'{direction:15} side of {tile_id} matches {other_direction:15} side of {other_tile_id}')
    print()
    grid = assemble_image(tiles)
    for row in grid:
        print(' '.join(str(tile_id for tile_id in row)))

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
        
def assemble_image(tiles):
    matching_directions = {tile_id: find_matching_directions(tile_id, tiles) for tile_id in tiles}
    # for tile_id, rotations in matching_rotations.items():
    #     print(f'Tile {tile_id}:')
    #     for rotation, directions in rotations.items():
    #         print(f'\tRotation: {rotation}')
    #         for direction, tile_ids in directions.items():
    #             print(f'\t\tDirection: {direction}')
    #             for tile_id in tile_ids:
    #                 print(f'\t\t\tTile {tile_id}')
    #     break
    remaining = set(tiles.keys())
    grid = [[]]
    num_tiles = len(tiles)
    for num_rows in divisors(num_tiles):
        num_columns = num_tiles // num_rows
        result = assemble_image_helper(tiles, matching_directions, remaining, grid, num_rows,
                                       num_columns, 0, 0)
        if result:
            return result
    raise Exception('No image found')

def assemble_image_helper(tiles, matching_directions, remaining, grid, num_rows, num_columns,
                          row_index, column_index):
    if not remaining:
        return grid
    offset = row_index * num_columns + column_index
    print(f'{" " * offset}remaining: {remaining}')
    print(f'{" " * offset}grid: {grid}')
    next_column_index = (column_index + 1) % num_columns
    next_row_index = row_index + 1 if next_column_index == 0 else row_index
    if next_column_index == 0:
        grid.append([])
    for tile_id in remaining:
        tile = tiles[tile_id]
        next_remaining = set(remaining)
        next_remaining.remove(tile_id)
        next_grid = [r[:] for r in grid]
        row = next_grid[row_index]
        for rotation in range(len(Direction)):
            if row_index > 0:
                above_tile_id, above_rotation = grid[row_index - 1][column_index]
                above_tile = tiles[above_tile_id]
                if not tiles_match(above_tile, above_rotation, tile, rotation, Direction.DOWN):
                    offset = row_index * num_columns + column_index
                    print(f'{" " * offset}Tile #{tile_id} does not match tile above, #{above_tile_id}')
                    continue
            if column_index > 0:
                left_tile_id, left_rotation = grid[row_index][column_index - 1]
                left_tile = tiles[left_tile_id]
                if not tiles_match(left_tile, left_rotation, tile, rotation, Direction.DOWN):
                    print(f'{" " * offset}Tile #{tile_id} does not match tile left, #{left_tile_id}')
                    continue
            print(f'{" " * offset}Adding tile #{tile_id} with rotation {rotation} at row ' + \
                  f'{row_index} and column {column_index}')
            row.append((tile_id, rotation))
            result = assemble_image_helper(tiles, matching_directions, next_remaining, next_grid,
                                           num_rows, num_columns, next_row_index, next_column_index)
            if result:
                return result
            del row[-1]

def tiles_match(tile1, rotation1, tile2, rotation2, direction):
    edge1 = get_rotated_edge(tile1, rotation1, direction)
    edge2 = get_rotated_edge(tile2, rotation2, direction.opposite())
    return edges_match(edge1, edge2)
    
def find_matching_directions(tile_id, tiles):
    tile = tiles[tile_id]
    matching = []
    for direction in Direction:
        edge = get_edge(tile, direction)
        for other_tile_id, other_tile in tiles.items():
            if other_tile_id == tile_id:
                continue
            for other_direction in Direction:
                other_edge = get_edge(other_tile, other_direction)
                if edges_match(edge, other_edge):
                    matching.append((direction, other_direction, other_tile_id))
    return matching

def edges_match(edge1, edge2):
    return all(x == y for x, y in zip(edge1, reversed(edge2)))

def get_edge(tile, direction):
    return get_rotated_edge(tile, 0, direction)

def get_rotated_edge(tile, rotation, direction):
    direction = add_wrapping_to_enum(direction, rotation)
    if direction == Direction.RIGHT:
        return [row[-1] for row in tile]
    elif direction == Direction.UP:
        return tile[0]
    elif direction == Direction.LEFT:
        return [row[0] for row in reversed(tile)]
    elif direction == Direction.DOWN:
        return list(reversed(tile[-1]))
    else:
        raise Exception(f'Non-exhaustive case for {direction}')


class Direction(Enum):
    RIGHT = auto()
    DOWN = auto()
    LEFT = auto()
    UP = auto()
    
    def rotation(self):
        return self.value - 1

    def opposite(self):
        return add_wrapping_to_enum(self, 2)

def divisors(num):
    return [x for x in range(1, num + 1) if num % x == 0]

def add_wrapping_to_enum(variant, add):
    cls = type(variant)
    zero_based_value = variant.value - 1
    zero_based_next_value = (zero_based_value + add) % len(cls)
    next_value = zero_based_next_value + 1
    return cls(next_value)

if __name__ == '__main__':
    main()
