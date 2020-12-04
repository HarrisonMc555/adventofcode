#!/usr/bin/env python3

INPUT_FILE = 'input04.txt'
REQUIRED_KEYS = ['byr', 'iyr', 'eyr', 'hgt', 'hcl', 'ecl', 'pid']

def main():
    with open(INPUT_FILE) as f:
        text = f.read()
    passports = parse_passports(text)
    print(count_valid_passports(REQUIRED_KEYS, passports))

def parse_passports(text):
    passports_text = text.split('\n\n')
    return [parse_passport(pt) for pt in passports_text]

def parse_passport(passport_text):
    pairs = passport_text.split()
    return dict(pair.split(':') for pair in pairs)

def count_valid_passports(required_keys, passports):
    valids = [is_valid_passport(required_keys, p) for p in passports]
    return valids.count(True)

def is_valid_passport(required_keys, passport):
    return all(key in passport for key in required_keys)

if __name__ == '__main__':
    main()
