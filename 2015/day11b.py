#!/usr/bin/env python3

INPUT = 'hepxcrrq'

import unittest
import re

DEBUG = False
def debug_print(*args, **kwargs):
    if DEBUG:
        print(*args, **kwargs)

def main():
    if 'INPUT' not in globals():
        unittest.main()
        return
    next_password = get_next_password(INPUT)
    print(get_next_password(next_password))

def is_valid_password(password):
    return all(rule(password) for rule in RULES)
    
def get_next_password(password):
    new_password = list(password)
    for i, c in enumerate(new_password):
        if c in 'iol':
            new_password[i] = get_next_char(c)
            new_password[i + 1:] = ['a'] * (len(new_password) - i - 1)
            break
    else:
        increment_password(new_password)
    while not is_valid_password(new_password):
        increment_password(new_password)
    return ''.join(new_password)

FIRST_CHAR = 'a'
MAX_CHAR = 'z'
def increment_password(password, start=0):
    for i in reversed(range(start, len(password))):
        # next_c = increment_char(password[i])
        # if next_c > MAX_CHAR:
        #     password[i] = FIRST_CHAR
        # else:
        #     password[i] = next_c
        #     break
        next_c = get_next_char(password[i])
        if next_c:
            password[i] = next_c
            break
        else:
            password[i] = FIRST_CHAR

def get_next_char(c):
    c = increment_char(c)
    while c in 'iol':
        c = increment_char(c)
    if c > MAX_CHAR:
        return None
    return c

def first_rule(password):
    for triplet in windows(password, 3):
        if is_increasing(triplet):
            return True
    return False

def second_rule(password):
    return not any(c in password for c in 'iol')

# THIRD_RULE_PATTERN = re.compile('.*(.)\1.*(.)\2.*')
def third_rule(password):
    num_doubles = 0
    i = 0
    while i < len(password) - 1:
        c1, c2 = password[i:i+2]
        if c1 == c2:
            num_doubles += 1
            if num_doubles == 2:
                return True
            i += 1
        i += 1
    return False

RULES = [first_rule, second_rule, third_rule]

def is_increasing(text):
    for c1, c2 in windows(text, 2):
        if increment_char(c1) != c2:
            return False
    return True

def increment_char(c):
    return chr(ord(c) + 1)

def windows(lst, window_length):
    for i in range(len(lst) - window_length + 1):
        yield lst[i:i + window_length]

PASSWORD_TO_VALID = [
    ('hijklmmn', False),
    ('abbceffg', False),
    ('abbcegjk', False),
    ('abcdffaa', True),
    ('ghjaabcc', True),
    ('ghjaaaaa', False),
]
PASSWORD_TO_NEXT_PASSWORD = [
    ('abcdefgh', 'abcdffaa'),
    ('ghijklmn', 'ghjaabcc'),
]
INCREMENTING_PASSWORDS = [
    ['abc', 'abd', 'abe', 'abf'],
    ['aby', 'abz', 'aca', 'acb'],
    ['azy', 'azz', 'baa', 'bab'],
    ['abi', 'abj', 'abk'],
    ['aic', 'aja']
]
class Test(unittest.TestCase):
    def test_windows(self):
        lst = [1, 2, 3, 4, 5]
        self.assertEqual(list(windows(lst, 1)), [[1], [2], [3], [4], [5]])
        self.assertEqual(list(windows(lst, 2)), [[1, 2], [2, 3], [3, 4], [4, 5]])
        self.assertEqual(list(windows(lst, 3)), [[1, 2, 3], [2, 3, 4], [3, 4, 5]])
        self.assertEqual(list(windows(lst, 4)), [[1, 2, 3, 4], [2, 3, 4, 5]])
        self.assertEqual(list(windows(lst, 5)), [[1, 2, 3, 4, 5]])

    def test_valid(self):
        password1 = 'hijklmmn'
        self.assertTrue(first_rule(password1), password1)
        self.assertFalse(second_rule(password1), password1)

        password2 = 'abbceffg'
        self.assertTrue(third_rule(password2), password2)
        self.assertFalse(first_rule(password2), password2)

        password3 = 'abbcegjk'
        self.assertFalse(third_rule(password3), password3)
        
        for password, is_valid in PASSWORD_TO_VALID:
            self.assertEqual(is_valid_password(list(password)), is_valid,
                             password)

    # def test_increment_password(self):
    #     for increments in INCREMENTING_PASSWORDS:
    #         cur_password = list(increments[0])
    #         for next_password in increments[1:]:
    #             increment_password(cur_password)
    #             self.assertEqual(list(next_password), cur_password,
    #                              next_password)

    def test_next_password(self):
        for password, next_password in PASSWORD_TO_NEXT_PASSWORD:
            self.assertEqual(get_next_password(password), next_password,
                             f'{password} -> {next_password}')

if __name__ == '__main__':
    main()
