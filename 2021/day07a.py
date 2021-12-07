#!/usr/bin/env python3

INPUT_FILE = 'input07.txt'
# INPUT_FILE = 'example07.txt'

def main():
    text = get_text(INPUT_FILE)
    positions = parse_text(text)
    least_fuel = get_least_fuel(positions)
    print(least_fuel)

def get_least_fuel(positions):
    smallest = min(positions)
    biggest = max(positions)
    least_fuel = biggest - smallest
    return min(get_fuel(positions, target_position)
               for target_position in range(smallest, biggest + 1))

def get_fuel(positions, target_position):
    return sum(abs(position - target_position)
               for position in positions)

def parse_text(text):
    return [int(word) for word in text.split(',')]

def get_text(filename):
    with open(filename) as f:
        return f.read().strip()

if __name__ == '__main__':
    main()
