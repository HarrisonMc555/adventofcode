#!/usr/bin/env python3

def solve(polymer):
    polymer = list(polymer)
    i = 0
    while i < len(polymer) - 1:
        unit1, unit2 = polymer[i:i+2]
        if is_opposite(unit1, unit2):
            del polymer[i:i+2]
            i = max(i-1, 0)
        else:
            i += 1
    return len(polymer)

def is_opposite(unit1, unit2):
    return unit1.lower() == unit2.lower() and unit1 != unit2

def get_input():
    return input()

def main():
    polymer = get_input()
    print(solve(polymer))

if __name__ == '__main__':
    main()
