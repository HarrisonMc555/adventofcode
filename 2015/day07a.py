#!/usr/bin/env python3

INPUT_FILE = 'input07.txt'

import unittest
import re
from abc import ABC, abstractmethod

DEBUG = False
def debug_print(*args, **kwargs):
    if DEBUG:
        print(*args, **kwargs)

def main():
    if 'INPUT_FILE' not in globals():
        unittest.main()
        return
    lines = get_lines(INPUT_FILE)
    print(run(lines))

def run(lines):
    dest_to_command = parse_lines(lines)
    dest_to_value = {}
    return get_signal_for(dest_to_command, dest_to_value, 'a')

def get_signal_for(dest_to_command, dest_to_value, dest):
    if dest in dest_to_value:
        return dest_to_value[dest]
    command, ops = dest_to_command[dest]
    def get(op):
        if op.isnumeric():
            return int(op)
        else:
            op_value = get_signal_for(dest_to_command, dest_to_value, op)
            dest_to_value[op] = op_value
            return op_value
    if command == 'RAW':
        op = ops[0]
        if op.isnumeric():
            return_value = int(op)
        else:
            return_value = get(op)
        debug_print(f'{dest} is RAW {ops[0]} = {return_value}')
        # return int(ops[0])
        return return_value
    elif command == 'NOT':
        op_value = get(ops[0])
        return_value = ~op_value & 0xFFFF
        debug_print(f'{dest} is NOT {ops[0]} (NOT {op_value}) = {return_value}')
        return return_value
        # return ~get(ops[0])
    elif command == 'AND':
        op1_value = get(ops[0])
        op2_value = get(ops[1])
        debug_print(f'{dest} is {ops[0]} AND {ops[1]} = ' + \
              f'{op1_value} AND {op2_value} = ' + \
              f'{op1_value & op2_value}')
        # return get(ops[0]) & get(ops[1])
        return op1_value & op2_value
    elif command == 'OR':
        op1_value = get(ops[0])
        op2_value = get(ops[1])
        debug_print(f'{dest} is {ops[0]} OR {ops[1]} = ' + \
              f'{op1_value} OR {op2_value} = ' + \
              f'{op1_value | op2_value}')
        # return get(ops[0]) | get(ops[1])
        return op1_value | op2_value
    elif command == 'LSHIFT':
        op1_value = get(ops[0])
        op2_value = int(ops[1])
        debug_print(f'{dest} is {ops[0]} LSHIFT {ops[1]} = ' + \
              f'{op1_value} LSHIFT {op2_value} = ' + \
              f'{op1_value << op2_value}')
        # return get(ops[0]) << int(ops[1])
        return op1_value << op2_value
    elif command == 'RSHIFT':
        op1_value = get(ops[0])
        op2_value = int(ops[1])
        debug_print(f'{dest} is {ops[0]} RSHIFT {ops[1]} ' + \
              f'{op1_value} RSHIFT {op2_value} = ' + \
              f'{op1_value >> op2_value}')
        # return get(ops[0]) >> int(ops[1])
        return op1_value >> op2_value
    else:
        raise Exception(f'Unrecognized command: {command}')

BINARY_OP_PATTERN = re.compile(r'(\w+) (AND|OR|LSHIFT|RSHIFT) (\w+)')
NOT_PATTERN = re.compile(r'NOT (\w+)')
RAW_PATTERN = re.compile(r'(\w+)')
def parse_command(command):
    binary_op_match = BINARY_OP_PATTERN.match(command)
    if binary_op_match:
        op1, cmd, op2 = binary_op_match.groups()
        return (cmd, [op1, op2])
    not_match = NOT_PATTERN.match(command)
    if not_match:
        op = not_match.groups()[0]
        return ('NOT', [op])
    raw_match = RAW_PATTERN.match(command)
    if raw_match:
        op = raw_match.groups()[0]
        return ('RAW', [op])
    raise Exception(f'Unrecognized command string: "{command}"')

def parse_lines(lines):
    gates = [parse_line(line.strip()) for line in lines]
    return {dest: parse_command(command) for command, dest in gates}

LINE_RE = re.compile(r'(.*) -> (\w+)')
def parse_line(line):
    try:
        return LINE_RE.match(line).groups()
    except Exception as e:
        print(f'Error parsing line "{line}"')
        raise e

def get_lines(filename):
    with open(filename) as f:
        return f.readlines()

EXAMPLE_LINES = [
    '123 -> x',
    '456 -> y',
    'x AND y -> d',
    'x OR y -> e',
    'x LSHIFT 2 -> f',
    'y RSHIFT 2 -> g',
    'NOT x -> h',
    'NOT y -> i',
]
DEST_TO_SIGNAL = {
    'd': 72,
    'e': 507,
    'f': 492,
    'g': 114,
    'h': 65412,
    'i': 65079,
    'x': 123,
    'y': 456,
}
class Test(unittest.TestCase):
    def test_example(self):
        dest_to_command = parse_lines(EXAMPLE_LINES)
        # debug_print(f'dest_to_command: {dest_to_command}')
        for dest, signal in DEST_TO_SIGNAL.items():
            # debug_print(f'Getting signal for {dest}')
            self.assertEqual(get_signal_for(dest_to_command, {}, dest), signal)
            # debug_print()

if __name__ == '__main__':
    main()
