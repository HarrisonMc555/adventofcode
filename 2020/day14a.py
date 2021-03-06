#!/usr/bin/env python3

import re
import abc

INPUT_FILE = 'input14.txt'
# INPUT_FILE = 'example.txt'

def main():
    lines = get_lines(INPUT_FILE)
    commands = parse_lines(lines)
    program = Program()
    program.run_commands(commands)
    print(sum(program.memory.values()))

def get_lines(filename):
    with open(filename) as f:
        return [line.strip() for line in f.readlines() if line]

def parse_lines(lines):
    return [parse_command(line) for line in lines]

class Program:
    def __init__(self):
        self.mask = 'X' * 36
        self.memory = {}

    def run_commands(self, commands):
        # print(f'Before running commands, mask is {self.mask} and memory is:')
        # for address, value in self.memory.items():
        #     print(f'\tmem[{address}] = {value}')
        # print()
        for command in commands:
            command.run(self)
            # print(f'Now, mask is {self.mask} and memory is:')
            # for address, value in self.memory.items():
            #     print(f'\tmem[{address}] = {value}')
            # print()

class Command:
    @abc.abstractmethod
    def run(self, program):
        pass

class Mask(Command):
    def __init__(self, mask):
        self.mask = mask

    def run(self, program):
        # print(f'Running: {self}')
        program.mask = self.mask

    MASK_RE = re.compile('mask = ([X01]{36})')
    def from_str(s):
        match = Mask.MASK_RE.match(s)
        if not match:
            raise Exception(f'Mask input: "{s}" did not match pattern')
        mask = match.group(1)
        return Mask(mask)

    def __str__(self):
        return f'mask = {self.mask}'

class Mem(Command):
    def __init__(self, address, value):
        self.address = address
        self.value = value

    def run(self, program):
        # print(f'Running: {self}')
        effective_value = get_effective_value(self.value, program.mask)
        # print(f'Effective value from mask:{program.mask} is {effective_value}')
        program.memory[self.address] = effective_value

    MEM_RE = re.compile('mem\[(\d+)\] = (\d+)')
    def from_str(s):
        match = Mem.MEM_RE.match(s)
        if not match:
            raise Exception(f'Mem input: "{s}" did not match pattern')
        address = int(match.group(1))
        value = int(match.group(2))
        return Mem(address, value)

    def __str__(self):
        return f'mem[{self.address}] = {self.value}'

def parse_command(line):
    if line.startswith('mask'):
        return Mask.from_str(line)
    elif line.startswith('mem'):
        return Mem.from_str(line)
    else:
        raise Exception('Invalid command: {line}')

def get_effective_value(value, mask):
    for index, flag in enumerate(reversed(mask)):
        if flag == '1':
            value |= 1 << index
        elif flag == '0':
            value &= ~(1 << index)
        elif flag == 'X':
            pass
        else:
            raise Exception(f'Invalid flag: {flag} at index {index} from the right')
    return value

if __name__ == '__main__':
    main()
