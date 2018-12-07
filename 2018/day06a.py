#!/usr/bin/env python3
#pylint: disable=invalid-name

import sys
from collections import Counter

def find_bounds(coordinates):
    xcoords, ycoords = [x for x, _ in coordinates], [y for _, y in coordinates]
    xbounds = min(xcoords), max(xcoords)
    ybounds = min(ycoords), max(ycoords)
    return xbounds, ybounds

def create_grid(bounds, named_coordinates):
    (xmin, xmax), (ymin, ymax) = bounds
    return [[get_closest((i, j), named_coordinates)[0]
             for j in range(ymin, ymax + 1)]
            for i in range(xmin, xmax + 1)]

def get_closest(start, named_coordinates):
    first = named_coordinates[0]
    best_name = first[0]
    best_distance = manhattan_distance(first[1], start)
    for name, coordinate in named_coordinates:
        distance = manhattan_distance(coordinate, start)
        if distance < best_distance:
            best_name = name
            best_distance = distance
    return best_name, best_distance

def manhattan_distance(start, finish):
    x1, y1 = start
    x2, y2 = finish
    xdistance = abs(x2 - x1)
    ydistance = abs(y2 - y1)
    return xdistance + ydistance

def get_num_min_distances(grid):
    flat_grid = flatten(grid)
    return Counter(flat_grid)

def remove_edges(num_min_distances, grid):
    first_row, last_row = grid[0], grid[-1]
    first_col, last_col = [row[0] for row in grid], [row[-1] for row in grid]
    edge_names = set(flatten([first_row, last_row, first_col, last_col]))
    return {name: num for name, num in num_min_distances.items()
            if name not in edge_names}

def name_coordinates(coordinates):
    return list(zip(infinite_counter(), coordinates))

def count_equal(enumerable, value):
    return count(enumerable, lambda x: x == value)

def count(enumerable, fun):
    return sum(1 for x in enumerable if fun(x))

def infinite_counter(start=0):
    num = start
    while True:
        yield num
        num += 1

def flatten(nested):
    return [value for inner in nested for value in inner]

def parse_coordinates(line):
    return tuple(int(x) for x in line.split(','))

def solve(coordinates):
    named_coordinates = name_coordinates(coordinates)
    bounds = find_bounds(coordinates)
    grid = create_grid(bounds, named_coordinates)
    item = grid
    while isinstance(item, list):
        item = item[0]
    num_min_distances = get_num_min_distances(grid)
    num_min_distances = remove_edges(num_min_distances, grid)
    return max(num_min_distances.values())

def get_input():
    return [parse_coordinates(line.strip()) for line in sys.stdin.readlines()]

def main():
    coordinates = get_input()
    print(solve(coordinates))

if __name__ == '__main__':
    main()
