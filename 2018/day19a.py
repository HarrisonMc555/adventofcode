#!/usr/bin/env python3
#pylint: disable=invalid-name

import copy

################################################################################
# Run
################################################################################
def solve(examples):
    num_matching_funs = [num_funs_matching_example(e) for e in examples]
    return count(num_matching_funs, lambda n: n >= 3)

def num_funs_matching_example(example):
    before, instruction, after = example
    num_matching = 0
    for fun in FUNCTIONS:
        result = run_example_fun(before, instruction, fun)
        if result == after:
            num_matching += 1
    return num_matching

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

def get_input():
    ip_reg = get_ip_reg()
    instructions = get_all_instructions()
    return ip_reg, instructions

def get_ip_reg():
    pass

def get_all_instructions():
    return [parse_instruction(line.strip()) for line in sys.stdin.readlines()]

def parse_instruction(line):
    return tuple(int(w) for w in line.split(' '))

def get_ints(string, sep=' '):
    return to_ints(string.split(sep))

def to_ints(strings):
    return [int(s) for s in strings]

def main():
    examples = get_input()
    print(solve(examples))

if __name__ == '__main__':
    main()
