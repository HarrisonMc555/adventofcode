#!/usr/bin/env python3

INPUT_FILE = 'input03.txt'
# INPUT_FILE = 'example03.txt'

def main():
    lines = get_lines(INPUT_FILE)
    bit_lines = parse_lines(lines)
    common_bits = get_common_bits(bit_lines)
    uncommon_bits = [not bit for bit in common_bits]
    gamma_rate = to_decimal(common_bits)
    epsilon_rate = to_decimal(uncommon_bits)
    print(gamma_rate * epsilon_rate)

def to_decimal(bits):
    total = 0
    for bit in bits:
        total *= 2
        if bit:
            total += 1
    return total

def get_common_bits(lines):
    num_bits = len(lines[0])
    return [find_common_bit(lines, i) for i in range(num_bits)]

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
