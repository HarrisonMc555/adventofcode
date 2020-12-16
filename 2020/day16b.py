#!/usr/bin/env python3

import re

INPUT_FILE = 'input16.txt'
# INPUT_FILE = 'example16b.txt'

def main():
    text = get_text(INPUT_FILE)
    rules, your_ticket, nearby_tickets = parse_text(text)
    # print_parsed_info(rules, your_ticket, nearby_tickets)
    valid_tickets = [ticket for ticket in nearby_tickets if ticket_could_be_valid(ticket, rules)]
    ordered_rules = find_rule_order(valid_tickets, rules)
    # for i, rule in enumerate(ordered_rules):
    #     print(f'{i}: {rule}')
    departure_rule_indices = [i for i, rule in enumerate(ordered_rules) if
                              rule[0].startswith('departure')]
    print(product(your_ticket[i] for i in departure_rule_indices))

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

def ticket_could_be_valid(ticket, rules):
    return all(any(rule_matches(rule, value) for rule in rules) for value in ticket)

def rule_matches(rule, value):
    _, min1, max1, min2, max2 = rule
    return min1 <= value <= max1 or min2 <= value <= max2

def find_rule_order(nearby_tickets, rules):
    num_rules = len(rules)
    rules = set(rules)
    index_to_rules = {}
    available_indices = set(range(num_rules))
    # print(f'nearby_tickets: {nearby_tickets}')
    # print()
    while len(index_to_rules) < num_rules:
        # print(f'index_to_rules: {index_to_rules}')
        # print(f'available_indices: {available_indices}')
        # print()
        index, rule = find_index_and_rule(nearby_tickets, rules, available_indices)
        index_to_rules[index] = rule
        rules.remove(rule)
        available_indices.remove(index)
    return [index_to_rules[i] for i in range(num_rules)]

def find_index_and_rule(nearby_tickets, rules, available_indices):
    for rule in rules:
        possible_indices = [i for i in available_indices if
                            rule_could_be_for_index(nearby_tickets, rule, i)]
        # print(f'\tfor rule: {rule}, possible_indices: {possible_indices}')
        if len(possible_indices) == 1:
            return possible_indices[0], rule
    raise Exception(f'No index for rule found')

def rule_could_be_for_index(nearby_tickets, rule, i):
    return all(rule_matches(rule, ticket[i]) for ticket in nearby_tickets)

def flatten(list_of_lists):
    return [x for l in list_of_lists for x in l]

def product(nums):
    result = 1
    for num in nums:
        result *= num
    return result

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
