#!/usr/bin/env python3

INPUT_FILE = 'input03.txt'
# INPUT_FILE = 'example03.txt'

def main():
    lines = get_lines(INPUT_FILE)
    bit_lines = parse_lines(lines)
    oxygen = to_decimal(find_rating(bit_lines, False))
    c02 = to_decimal(find_rating(bit_lines, True))
    print(oxygen * c02)

def to_decimal(bits):
    total = 0
    for bit in bits:
        total *= 2
        if bit:
            total += 1
    return total

def find_rating(lines, flip_bit):
    num_bits = len(lines[0])
    for i in range(num_bits):
        if len(lines) == 1:
            return lines[0]
        bit = find_common_bit(lines, i)
        desired_bit = bit ^ flip_bit
        lines = [line for line in lines
                 if line[i] == desired_bit]
    if len(lines) != 1:
        raise Exception('Could not filter')
    return lines[0]

def find_common_bit(lines, index):
    num_true = 0
    num_false = 0
    for line in lines:
        if line[index]:
            num_true += 1
        else:
            num_false += 1
    return num_true >= num_false

def parse_lines(lines):
    return [parse_line(line) for line in lines]

def parse_line(line):
    return [c == '1' for c in line]

def get_lines(filename):
    with open(filename) as f:
        return [line.strip() for line in f.readlines() if line]

if __name__ == '__main__':
    main()
