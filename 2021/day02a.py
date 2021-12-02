#!/usr/bin/env python3

INPUT_FILE = 'input02.txt'
# INPUT_FILE = 'example02.txt'

def main():
    lines = get_lines(INPUT_FILE)
    commands = parse_lines(lines)
    horizontal, depth = run_commands(commands)
    print(horizontal * depth)

def run_commands(commands):
    horizontal = 0
    depth = 0
    for command, num in commands:
        if command == 'forward':
            horizontal += num
        elif command == 'down':
            depth += num
        elif command == 'up':
            depth -= num
        else:
            raise Exception('Unrecognized command: ' + command)
    return horizontal, depth
        

def parse_lines(lines):
    return [parse_line(line) for line in lines]

def parse_line(line):
    command, num_string = line.split(' ')
    num = int(num_string)
    return command, num

def get_lines(filename):
    with open(filename) as f:
        return [line.strip() for line in f.readlines() if line]

if __name__ == '__main__':
    main()
