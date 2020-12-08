#!/usr/bin/env python3

from enum import Enum, auto

INPUT_FILE = 'input08.txt'
# INPUT_FILE = 'example.txt'

def main():
    lines = get_lines(INPUT_FILE)
    instructions = parse_lines(lines)
    program = Program(instructions)
    program.run_until_loop()
    print(program.accumulator)

def get_lines(filename):
    with open(filename) as f:
        return [line.strip() for line in f.readlines() if line]

def parse_lines(lines):
    return [parse_line(line) for line in lines]

def parse_line(line):
    opcode, arg = line.split(' ')
    opcode = Operation.from_str(opcode)
    arg = int(arg)
    return opcode, arg

class Operation(Enum):
    NOP = auto()
    ACC = auto()
    JMP = auto()

    def from_str(s):
        if s == 'nop':
            return Operation.NOP
        elif s == 'acc':
            return Operation.ACC
        elif s == 'jmp':
            return Operation.JMP
        else:
            raise Exception(f'Invalid opcode {s}')

class Program:
    def __init__(self, instructions):
        self.instructions = instructions
        self.accumulator = 0
        self.address = 0

    def step(self):
        op, arg = self.instructions[self.address]
        self.run_instruction(op, arg)

    def run_instruction(self, op, arg):
        self.next_address = self.address + 1
        if op == Operation.NOP:
            self.nop()
        elif op == Operation.ACC:
            self.acc(arg)
        elif op == Operation.JMP:
            self.jmp(arg)
        else:
            raise Exception(f'Unsupported operation {op}')
        self.address = self.next_address

    def nop(self):
        pass

    def acc(self, arg):
        self.accumulator += arg

    def jmp(self, arg):
        self.next_address = self.address + arg

    def run_until_loop(self):
        seen_addresses = set()
        while self.address not in seen_addresses:
            seen_addresses.add(self.address)
            self.step()
    
    def __str__(self):
        return f'accumulator: {self.accumulator}, address: {self.address}, # of instructions: {len(self.instructions)}'


if __name__ == '__main__':
    main()
