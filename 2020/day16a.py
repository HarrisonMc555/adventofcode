#!/usr/bin/env python3

import re

INPUT_FILE = 'input16.txt'
# INPUT_FILE = 'example16.txt'

def main():
    text = get_text(INPUT_FILE)
    rules, your_ticket, nearby_tickets = parse_text(text)
    # print_parsed_info(rules, your_ticket, nearby_tickets)
    invalids = find_all_invalid_fields(nearby_tickets, rules)
    # print(invalids)
    print(sum(invalids))

def get_text(filename):
    with open(filename) as f:
        return f.read().strip()

def parse_text(lines):
    rules_text, your_text, nearby_text = lines.split('\n\n')
    rules = parse_rules(rules_text)
    your = parse_your_ticket(your_text)
    nearby = parse_nearby_tickets(nearby_text)
    return rules, your, nearby

def parse_rules(rules_text):
    return [parse_rule(line) for line in rules_text.split('\n')]

RULE_RE = re.compile(r'([a-zA-Z0-9 ]+): (\d+)-(\d+) or (\d+)-(\d+)')
def parse_rule(line):
    groups = RULE_RE.match(line).groups()
    name = groups[0]
    num_groups = groups[1:]
    nums = [int(x) for x in num_groups]
    return (name, *nums)

def parse_your_ticket(text):
    ticket_line = text.split('\n')[1]
    return parse_ticket(ticket_line)

def parse_nearby_tickets(text):
    lines = text.split('\n')
    return [parse_ticket(line) for line in lines[1:]]

def parse_ticket(line):
    return [int(x) for x in line.split(',')]

def find_all_invalid_fields(nearby_tickets, rules):
    return flatten([find_invalid_fields(ticket, rules) for ticket in nearby_tickets])

def find_invalid_fields(ticket, rules):
    return [value for value in ticket if not any(rule_matches(rule, value) for rule in rules)]

def rule_matches(rule, value):
    _, min1, max1, min2, max2 = rule
    return min1 <= value <= max1 or min2 <= value <= max2

def flatten(list_of_lists):
    return [x for l in list_of_lists for x in l]

def print_parsed_info(rules, your_ticket, nearby_tickets):
    print('rules:')
    for rule in rules:
        print(f'\t{rule}')
    print()
    print(f'your ticket: {your_ticket}')
    print()
    print('nearby_tickets:')
    for ticket in nearby_tickets:
        print(f'\t{ticket}')
    print()
    

if __name__ == '__main__':
    main()
