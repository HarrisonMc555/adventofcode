#!/usr/bin/env python3

# REAL = True

import unittest

POS_MIN_X = 94
POS_MAX_X = 151
BOUNDS_POS_X = POS_MIN_X, POS_MAX_X
RANGE_POS_X = range(POS_MIN_X, POS_MAX_X + 1)
POS_MIN_Y = -156
POS_MAX_Y = -103
BOUNDS_POS_Y = POS_MIN_Y, POS_MAX_Y
RANGE_POS_Y = range(POS_MIN_Y, POS_MAX_Y + 1)
def main():
    print(sum_one_to_n(abs(POS_MIN_Y) - 1))

def sum_one_to_n(num):
    return num * (num + 1) // 2

if __name__ == '__main__':
    main()
