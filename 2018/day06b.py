#!/usr/bin/env python3
#pylint: disable=invalid-name

import sys

def find_bounds(coordinates):
    xcoords, ycoords = [x for x, _ in coordinates], [y for _, y in coordinates]
    xbounds = min(xcoords), max(xcoords)
    ybounds = min(ycoords), max(ycoords)
    return xbounds, ybounds

def get_distance_sums(bounds, coordinates):
    (xmin, xmax), (ymin, ymax) = bounds
    return [get_distance_sum((i, j), coordinates)
            for i in range(xmin, xmax + 1)
            for j in range(ymin, ymax + 1)]

def get_distance_sum(start, coordinates):
    return sum(manhattan_distance(start, coordinate)
               for coordinate in coordinates)

def manhattan_distance(start, finish):
    x1, y1 = start
    x2, y2 = finish
    xdistance = abs(x2 - x1)
    ydistance = abs(y2 - y1)
    return xdistance + ydistance

def count(enumerable, fun):
    return sum(1 for x in enumerable if fun(x))

def parse_coordinates(line):
    return tuple(int(x) for x in line.split(','))

LIMIT = 10000
def solve(coordinates):
    bounds = find_bounds(coordinates)
    distance_sums = get_distance_sums(bounds, coordinates)
    return count(distance_sums, lambda x: x < LIMIT)

def get_input():
    return [parse_coordinates(line.strip()) for line in sys.stdin.readlines()]

def main():
    coordinates = get_input()
    print(solve(coordinates))

if __name__ == '__main__':
    main()
