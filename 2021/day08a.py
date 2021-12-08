#!/usr/bin/env python3

INPUT_FILE = 'input08.txt'
# INPUT_FILE = 'example08.txt'

# 0: abc_efg (6)
# 1: __c__f_ (2)
# 2: a_cde_g (6)
# 3: a_cd_fg (5)
# 4: _bcd_f_ (4)
# 5: ab_d_fg (5)
# 6: ab_defg (6)
# 7: a_c__f_ (3)
# 8: abcdefg (7)
# 9: abcd_fg (6)

def main():
    lines = get_lines(INPUT_FILE)
    parsed_lines = parse_lines(lines)
    print(count_1478_lines(parsed_lines))

def count_1478_lines(parsed_lines):
    return sum(count_1478(output_values) for _, output_values in parsed_lines)

LEN_1478 = {2, 4, 3, 7}
def count_1478(output_values):
    return sum(1 for output in output_values if len(output) in LEN_1478)

def parse_lines(lines):
    return [parse_line(line) for line in lines]

def parse_line(line):
    unique_patterns_text, output_values_text = line.split('|')
    unique_patterns = unique_patterns_text.split()
    output_values = output_values_text.split()
    return unique_patterns, output_values

def get_lines(filename):
    with open(filename) as f:
        return f.readlines()

if __name__ == '__main__':
    main()
