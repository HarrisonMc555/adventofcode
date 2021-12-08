#!/usr/bin/env python3

INPUT_FILE = 'input08.txt'
# INPUT_FILE = 'example08.txt'
EXAMPLE_TEXT = 'acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf'

from collections import Counter

# 0: abc_efg (6)
# 1: __c__f_ (2)*
# 2: a_cde_g (6)
# 3: a_cd_fg (5)
# 4: _bcd_f_ (4)*
# 5: ab_d_fg (5)
# 6: ab_defg (6)
# 7: a_c__f_ (3)*
# 8: abcdefg (7)*
# 9: abcd_fg (6)
#    8687497
#     *  **
#     *^ **
#    ^*^ **
#    ^*^ **

def main():
    lines = get_lines(INPUT_FILE)
    # lines = [EXAMPLE_TEXT]
    parsed_lines = parse_lines(lines)
    print(sum(get_num(unique, output) for unique, output in parsed_lines))

def get_num(unique_patterns, output_values):
    mapping = get_mapping(unique_patterns)
    digits = [segments_to_num(mapping, mixed_segments)
              for mixed_segments in output_values]
    return digits_to_num(digits)

def digits_to_num(digits):
    result = 0
    for digit in digits:
        result *= 10
        result += digit
    return result

def segments_to_num(mapping, mixed_segments):
    real_segments = [map_segments(mapping, mixed_segment)
                     for mixed_segment in mixed_segments]
    segments_to_num = {
        'abcefg': 0,
        'cf': 1,
        'acdeg': 2,
        'acdfg': 3,
        'bcdf': 4,
        'abdfg': 5,
        'abdefg': 6,
        'acf': 7,
        'abcdefg': 8,
        'abcdfg': 9,
    }
    return segments_to_num[''.join(sorted(real_segments))]
    # return [segments_to_num[segments] for segments in real_segments]

def map_segments(mapping, mixed_segment):
    return ''.join(sorted(mapping[s] for s in mixed_segment))

def get_mapping(unique_patterns):
    segment_appearances = Counter()
    for pattern in unique_patterns:
        segment_appearances.update(pattern)
    mapping = {}
    for segment, count in segment_appearances.items():
        if count == 6:
            mapping[segment] = 'b'
        elif count == 4:
            mapping[segment] = 'e'
        elif count == 9:
            mapping[segment] = 'f'
    if len(mapping) != 3:
        raise Exception(f'Could not find b, e, and f. mapping: {mapping}')
    pattern_length_to_segment = [
        (2, 'c'),
        (3, 'a'),
        (4, 'd'),
    ]
    for pattern_length, segment in pattern_length_to_segment:
        pattern = find_pattern_with_length(unique_patterns, pattern_length)
        unknown_segment = find_unknown_segment(mapping, pattern)
        mapping[unknown_segment] = segment
    remaining_mixed = [c for c in 'abcdefg' if c not in mapping]
    if len(remaining_mixed) != 1:
        raise Exception('Not exactly one remaining mixed: {remaining_mixed}')
    last_remaining_mixed = remaining_mixed[0]
    mapping[last_remaining_mixed] = 'g'
    # print('mapping:')
    # for mixed, real in mapping.items():
    #     print(f'\t{mixed}: {real}')
    return mapping

def find_pattern_with_length(unique_patterns, length):
    patterns = [p for p in unique_patterns if len(p) == length]
    if len(patterns) != 1:
        raise Exception(f'Not exactly one pattern with length {length}: {one_patterns}')
    return patterns[0]

def find_unknown_segment(mapping, pattern):
    unknown_segments = [c for c in pattern if c not in mapping]
    if len(unknown_segments) != 1:
        raise Exception(f'Not exactly one unkown segment for {pattern}: {unknown_segments}')
    return unknown_segments[0]

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
