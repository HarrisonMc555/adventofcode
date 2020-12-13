#!/usr/bin/env python3

INPUT_FILE = 'input13.txt'
# INPUT_FILE = 'example.txt'

def main():
    lines = get_lines(INPUT_FILE)
    earliest_time, bus_ids = parse_lines(lines)
    bus_id = find_earliest_bus(earliest_time, bus_ids)
    wait_time = get_wait_time(earliest_time, bus_id)
    print(bus_id * wait_time)

def get_lines(filename):
    with open(filename) as f:
        return [line.strip() for line in f.readlines() if line]

def parse_lines(lines):
    earliest_time = int(lines[0])
    bus_strings = lines[1].split(',')
    bus_ids = [int(s) for s in bus_strings if s != 'x']
    return earliest_time, bus_ids

def find_earliest_bus(earliest_time, bus_ids):
    best_bus_id = bus_ids[0]
    best_wait_time = get_wait_time(earliest_time, best_bus_id)
    # print(f'Initial best_bus_id:{best_bus_id}, best_wait_time:{best_wait_time}')
    for bus_id in bus_ids:
        wait_time = get_wait_time(earliest_time, bus_id)
        # print(f'Considering bus_id:{bus_id}, wait_time:{wait_time}')
        if wait_time < best_wait_time:
            best_bus_id = bus_id
            best_wait_time = wait_time
            # print(f'\tNew best_bus_id:{best_bus_id}, best_wait_time:{best_wait_time}')
    # print(f'Returning: {best_bus_id}')
    return best_bus_id
    
def get_wait_time(earliest_time, bus_id):
    time_over = earliest_time % bus_id
    if time_over == 0:
        return time_over
    else:
        return bus_id - time_over


if __name__ == '__main__':
    main()
