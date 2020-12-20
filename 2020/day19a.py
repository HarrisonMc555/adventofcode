#!/usr/bin/env python3

import re
import abc
import unittest

INPUT_FILE = 'input19.txt'
# INPUT_FILE = 'example19a.txt'

def main():
    # unittest.main()
    text = get_text(INPUT_FILE)
    rules, messages = parse_text(text)
    # print('rules:')
    # for i in range(len(rules)):
    #     print(f'{i}: {rules[i]}')
    # print()
    # for message in messages:
    #     print(f'{message}: {is_valid(message, rules)}')
    # for message in ['a', 'b', 'aa', 'bb']:
    #     print(message)
    #     for i in range(len(rules)):
    #         rule = rules[i]
    #         print(f'\trule {i}: {matches(message, rule, rules)} <- {rule}')
    valids = [is_valid(message, rules) for message in messages]
    print(valids.count(True))

def get_text(filename):
    with open(filename) as f:
        return f.read().strip()

def parse_text(text):
    rules_text, messages_text = text.split('\n\n')
    return parse_rules(rules_text), parse_messages(messages_text)

def parse_rules(text):
    return dict(parse_rule(line) for line in text.split('\n'))
    # return [parse_rule(line) for line in lines]

def parse_messages(text):
    return text.split('\n')

def is_valid(message, rules):
    return matches(message, rules[0], rules)

def matches(message, rule, rules):
    try:
        remaining = rule.match(message, rules)
        return remaining == ''
    except NoMatchException:
        return False
    
RULE_RE = re.compile(r'^(\d+): (.*)$')
LITERAL_RE = re.compile('^"(.)"$')
def parse_rule(line):
    num_s, rule_s = RULE_RE.match(line).groups()
    num = int(num_s)
    literal_match = LITERAL_RE.match(rule_s)
    if literal_match:
        return num, Literal(literal_match.group(1))
    rule_groups = [parse_rule_group(w) for w in rule_s.split(' | ')]
    return num, Compound(rule_groups)

def parse_rule_group(text):
    rule_ids = text.split(' ')
    return [int(ri) for ri in rule_ids]

class NoMatchException(Exception):
    pass

class Rule:
    @abc.abstractmethod
    def match(self, text, rules):
        pass

class Literal(Rule):
    def __init__(self, text):
        self.text = text

    def match(self, text, rules):
        if not text.startswith(self.text):
            raise NoMatchException()
        return text[len(self.text):]

    def __str__(self):
        return f"Literal('{self.text}')"

class Compound(Rule):
    def __init__(self, groups):
        self.groups = groups

    def match(self, text, rules):
        for group in self.groups:
            try:
                return self.match_group(text, group, rules)
            except:
                pass
        raise NoMatchException()

    def match_group(self, text, group, rules):
        for rule_id in group:
            rule = rules[rule_id]
            text = rule.match(text, rules)
        return text

    def __str__(self):
        return f'Compound({self.groups})'
        
class MatchTest(unittest.TestCase):
    def test_literal(self):
        rules = {}
        literal = Literal('a')
        self.assertEqual('', literal.match('a', rules))
        self.assertEqual('b', literal.match('ab', rules))
        self.assertEqual('bba', literal.match('abba', rules))
        self.assertTrue(matches('a', literal, rules))
        self.assertFalse(matches('b', literal, rules))
        self.assertFalse(matches('', literal, rules))

    def test_simple_comound_group(self):
        rules = {
            1: Literal('a'),
            2: Literal('b'),
        }
        compound = Compound([[1, 2], [2, 1]])
        self.assertEqual('', compound.match('ab', rules))
        self.assertEqual('', compound.match('ba', rules))
        self.assertEqual('ba', compound.match('abba', rules))
        self.assertEqual('cd', compound.match('abcd', rules))
        self.assertRaises(NoMatchException, compound.match, 'aa', rules)
        
if __name__ == '__main__':
    main()
