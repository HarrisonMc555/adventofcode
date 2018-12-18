#!/usr/bin/env python3
#pylint: disable=invalid-name

import sys
import re
import copy
from collections import defaultdict

################################################################################
# Run
################################################################################
def solve(examples, instructions):
    opcode_to_fun = match_opcode_to_funs(examples)
    registers = defaultdict(int)
    for instruction in instructions:
        run_instruction(instruction, registers, opcode_to_fun)
    assert 0 in registers
    return registers[0]

def run_instruction(instruction, registers, opcode_to_fun):
    opcode, a, b, c = instruction
    fun = opcode_to_fun[opcode]
    fun(a, b, c, registers)

NUM_OPCODES = 16
def match_opcode_to_funs(examples):
    mapping = {opcode: set(FUNCTIONS) for opcode in range(NUM_OPCODES)}
    for example in examples:
        matching_funs = get_matching_funs(example)
        opcode = get_opcode(example)
        mapping[opcode].intersection_update(matching_funs)
    mapping_final = {}
    while len(mapping_final) < NUM_OPCODES:
        funs_to_remove = set()
        something_changed = False
        for opcode, funs in mapping.items():
            if len(funs) == 1:
                fun = next(iter(funs))
                mapping_final[opcode] = fun
                funs_to_remove.add(fun)
                something_changed = True
        for opcode, funs in mapping.items():
            funs.difference_update(funs_to_remove)
        assert something_changed
    return mapping_final

def get_matching_funs(example):
    return [fun for fun in FUNCTIONS if fun_matches_example(fun, example)]

def get_opcode(example):
    _, instruction, _ = example
    opcode, _, _, _ = instruction
    return opcode

def fun_matches_example(fun, example):
    before, instruction, after = example
    result = run_example_fun(before, instruction, fun)
    return result == after

def run_example_fun(registers, instruction, fun):
    registers = copy.deepcopy(registers)
    _, a, b, c = instruction
    fun(a, b, c, registers)
    return registers

def count(enumerable, fun):
    return sum(1 for val in enumerable if fun(val))

################################################################################
# Instructions
################################################################################
def instruction_addr(a, b, c, registers):
    registers[c] = registers[a] + registers[b]

def instruction_addi(a, b, c, registers):
    registers[c] = registers[a] + b

def instruction_mulr(a, b, c, registers):
    registers[c] = registers[a] * registers[b]

def instruction_muli(a, b, c, registers):
    registers[c] = registers[a] * b

def instruction_banr(a, b, c, registers):
    registers[c] = registers[a] & registers[b]

def instruction_bani(a, b, c, registers):
    registers[c] = registers[a] & b

def instruction_borr(a, b, c, registers):
    registers[c] = registers[a] | registers[b]

def instruction_bori(a, b, c, registers):
    registers[c] = registers[a] | b

def instruction_setr(a, _b, c, registers):
    registers[c] = registers[a]

def instruction_seti(a, _b, c, registers):
    registers[c] = a

def instruction_gtir(a, b, c, registers):
    registers[c] = 1 if a > registers[b] else 0

def instruction_gtri(a, b, c, registers):
    registers[c] = 1 if registers[a] > b else 0

def instruction_gtrr(a, b, c, registers):
    registers[c] = 1 if registers[a] > registers[b] else 0

def instruction_eqir(a, b, c, registers):
    registers[c] = 1 if a == registers[b] else 0

def instruction_eqri(a, b, c, registers):
    registers[c] = 1 if registers[a] == b else 0

def instruction_eqrr(a, b, c, registers):
    registers[c] = 1 if registers[a] == registers[b] else 0

FUNCTIONS = [instruction_addr,
             instruction_addi,
             instruction_mulr,
             instruction_muli,
             instruction_banr,
             instruction_bani,
             instruction_borr,
             instruction_bori,
             instruction_setr,
             instruction_seti,
             instruction_gtir,
             instruction_gtri,
             instruction_gtrr,
             instruction_eqir,
             instruction_eqri,
             instruction_eqrr]

################################################################################
# Input
################################################################################

BEFORE_PATTERN = re.compile(r'Before: *\[(.*)\]')
AFTER_PATTERN = re.compile(r'After: *\[(.*)\]')
def get_input():
    examples = list(get_all_examples())
    instructions = get_all_instructions()
    return examples, instructions

def get_all_examples():
    while True:
        before_line = input().strip()
        instruction_line = input().strip()
        # after examlpes, there are two blank lines
        if not instruction_line:
            return
        after_line = input().strip()
        input() # consume blank line
        before = get_ints(BEFORE_PATTERN.match(before_line).groups()[0], ', ')
        instruction = tuple(get_ints(instruction_line, ' '))
        after = get_ints(AFTER_PATTERN.match(after_line).groups()[0], ', ')
        yield before, instruction, after

def get_all_instructions():
    return [parse_instruction(line.strip()) for line in sys.stdin.readlines()]

def parse_instruction(line):
    return tuple(int(w) for w in line.split(' '))

def get_ints(string, sep=' '):
    return to_ints(string.split(sep))

def to_ints(strings):
    return [int(s) for s in strings]

def main():
    examples, instructions = get_input()
    print(solve(examples, instructions))

if __name__ == '__main__':
    main()
