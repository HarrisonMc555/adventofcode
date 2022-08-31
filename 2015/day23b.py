#!/usr/bin/env python3

INPUT_FILE = 'input23.txt'

import unittest
from dataclasses import dataclass

DEBUG = False
def debug_print(*args, **kwargs):
    if DEBUG:
        print(*args, **kwargs)

def main():
    if 'INPUT_FILE' not in globals():
        unittest.main()
        return
    instructions = get_lines(INPUT_FILE)
    state = State.create(instructions)
    registers = state.run()
    print(registers['b'])

@dataclass
class State:
    registers: {str: int}
    instructions: [str]
    index: int

    @staticmethod
    def create(instructions):
        return State({'a': 1, 'b': 0}, instructions, 0)

    def run(self):
        while self.step():
            pass
        return self.registers

    def step(self):
        if self.index >= len(self.instructions):
            return False
        instruction = self.instructions[self.index]
        pieces = instruction.split(',')
        cmd, r = pieces[0].split(' ')
        if len(pieces) > 1:
            offset = int(pieces[1])
        if cmd == 'hlf':
            self.hlf(r)
        elif cmd == 'tpl':
            self.tpl(r)
        elif cmd == 'inc':
            self.inc(r)
        elif cmd == 'jmp':
            self.jmp(int(r))
        elif cmd == 'jie':
            self.jie(r, offset)
        elif cmd == 'jio':
            self.jio(r, offset)
        else:
            raise Exception(f'Unrecognized command {cmd}')
        return True

    def hlf(self, r):
        self.registers[r] //= 2
        self.index += 1

    def tpl(self, r):
        self.registers[r] *= 3
        self.index += 1

    def inc(self, r):
        self.registers[r] += 1
        self.index += 1

    def jmp(self, offset):
        self.index += offset

    def jie(self, r, offset):
        if is_even(self.registers[r]):
            self.index += offset
        else:
            self.index += 1

    def jio(self, r, offset):
        if self.registers[r] == 1:
            self.index += offset
        else:
            self.index += 1
        

def is_even(x):
    return x % 2 == 0

def get_lines(filename):
    with open(filename) as f:
        return [line.strip() for line in f.readlines()]

class Test(unittest.TestCase):
    def test_example(self):
        pass

if __name__ == '__main__':
    main()
