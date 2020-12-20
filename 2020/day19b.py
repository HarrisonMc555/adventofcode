#!/usr/bin/env python3

import re
import abc
import unittest

INPUT_FILE = 'input19.txt'
# INPUT_FILE = 'example19b.txt'

RECURSIVE_RULES = [8, 11]
ADDITIONAL_RULES = '''8: 42 | 42 8
11: 42 31 | 42 11 31'''
# ADDITIONAL_RULES = ''

def main():
    # unittest.main()
    text = get_text(INPUT_FILE)
    rules, messages = parse_text(text)
    # print('rules:')
    # for i in sorted(rules.keys()):
    #     print(f'{i}: {rules[i]}')
    # print()

    # print(is_valid('babbbbaabbbbbabbbbbbaabaaabaaa', rules))

    # for message in messages:
    #     print(f'{is_valid(message, rules)} {message}')
    # for message in ['a', 'b', 'aa', 'bb']:
    #     print(message)
    #     for i in sorted(rules.keys()):
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
    if ADDITIONAL_RULES:
        text = text + '\n' + ADDITIONAL_RULES
    return dict(parse_rule(line) for line in text.split('\n'))
    # return [parse_rule(line) for line in lines]

def parse_messages(text):
    return text.split('\n')

def is_valid(message, rules):
    return matches(message, rules[0], rules)

def matches(message, rule, rules):
    for remaining in rule.match(message, rules):
        if remaining == '':
            return True
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

level = 0
class Literal(Rule):
    def __init__(self, text):
        self.text = text

    def match(self, text, rules):
        global level
        if text.startswith(self.text):
            # print(f'{" " * level}text DOES match literal \'{self.text}\': {text}')
            yield text[len(self.text):]
        # else:
        #     print(f'{" " * level}text does NOT match literal \'{self.text}\': {text}')

    def __str__(self):
        return f"Literal('{self.text}')"

class Compound(Rule):
    def __init__(self, groups):
        self.groups = groups

    def match(self, text, rules):
        global level
        # print(f'{" " * level}trying compound {self} with text: {text}')
        level += 1

        is_rule_8 = False
        is_recursive = False
        if 8 in rules:
            rule_8 = rules[8]
            is_rule_8 = str(rule_8) == str(self)
            is_recursive = any(rules[i] == self for i in RECURSIVE_RULES)
        # if is_recursive:
        #     print(f'Using recursive rule {self}')
        # if is_rule_8:
        #     print('This is rule 8!!!')
        #     print(f'groups: {self.groups}')
        for group in self.groups:
            # if is_rule_8:
            #     print(f'Rule 8 group: {group}')
            # print(f'{" " * level}trying group {group} with text: {text}')
            try:
                # return self.match_group(text, group, rules)
                # yield from self.match_group(text, group, rules)
                yield from match_group(text, group, rules)
                # text = self.match_group(text, group, rules)
                level -= 1
                # if is_rule_8:
                #     print('Matched rule 8!')
                # return text
                # yield text
                # if not is_recursive:
                #     raise NoMatchException()
            except:
                # if is_rule_8:
                #     print('Did NOT match rule 8!')
                # print(f'{" " * (level + 1)}did NOT match group {group} with text: {text}')
                pass
        # print(f'{" " * level}did NOT match ANY group in {self} with text: {text}')
        # raise NoMatchException()

    # def match_group(self, text, group, rules):
    #     if not group:
    #         yield text
    #         raise StopIteration
    #     rule_id = group[0]
    #     rule = rules[rule_id]
    #     for next_text in rule.match(text, rules):
    #         yield from match_group(next_text)
    #         yield from self.match_group(next_text, group[1:], rules)

        # for rule_id in group:
        #     rule = rules[rule_id]
        #     for 
        #     text = rule.match(text, rules)
        # return text

    def __str__(self):
        return f'Compound({self.groups})'

def match_group(text, group, rules):
    if not group:
        yield text
    else:
        rule_id = group[0]
        rule = rules[rule_id]
        for next_text in rule.match(text, rules):
            yield from match_group(next_text, group[1:], rules)

class MatchTest(unittest.TestCase):
    def test_literal(self):
        rules = {}
        literal = Literal('a')
        self.assertEqual('', next(literal.match('a', rules)))
        self.assertEqual('b', next(literal.match('ab', rules)))
        self.assertEqual('bba', next(literal.match('abba', rules)))
        self.assertTrue(matches('a', literal, rules))
        self.assertFalse(matches('b', literal, rules))
        self.assertFalse(matches('', literal, rules))

    def test_simple_comound_group(self):
        rules = {
            1: Literal('a'),
            2: Literal('b'),
        }
        compound = Compound([[1, 2], [2, 1]])
        self.assertEqual('', next(compound.match('ab', rules)))
        self.assertEqual('', next(compound.match('ba', rules)))
        self.assertEqual('ba', next(compound.match('abba', rules)))
        self.assertEqual('cd', next(compound.match('abcd', rules)))
        self.assertRaises(StopIteration, next, compound.match('aa', rules))
        
if __name__ == '__main__':
    main()
