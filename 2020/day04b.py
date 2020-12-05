#!/usr/bin/env python3

import unittest
INPUT_FILE = 'input04.txt'
# INPUT_FILE = 'example.txt'


def main():
    # unittest.main()
    # return
    with open(INPUT_FILE) as f:
        text = f.read()
    passports = parse_passports(text)
    print(count_valid_passports(VALIDATORS, passports))


def parse_passports(text):
    passports_text = text.split('\n\n')
    return [parse_passport(pt) for pt in passports_text]


def parse_passport(passport_text):
    pairs = passport_text.split()
    return dict(pair.split(':') for pair in pairs)


def count_valid_passports(validators, passports):
    valids = [is_valid_passport(validators, p) for p in passports]
    return valids.count(True)


def is_valid_passport(validators, passport):
    for key, validator in validators.items():
        try:
            val = passport[key]
        except:
            return False
        if not validator(val):
            return False
    return True


def int_validator(min_val, max_val):
    def validator(val):
        try:
            x = int(val)
        except:
            return False
        return min_val <= x <= max_val
    return validator


# Validators
byr = int_validator(1920, 2002)
iyr = int_validator(2010, 2020)
eyr = int_validator(2020, 2030)

hgt_cm = int_validator(150, 193)
hgt_in = int_validator(59, 76)

CM_SUFFIX = 'cm'
IN_SUFFIX = 'in'


def hgt(val):
    if val.endswith(CM_SUFFIX):
        num = val[:-len(CM_SUFFIX)]
        return hgt_cm(num)
    elif val.endswith(IN_SUFFIX):
        num = val[:-len(IN_SUFFIX)]
        return hgt_in(num)
    else:
        return False


HCL_PREFIX = '#'
HCL_CODE_LEN = 6


def hcl(val):
    if not val.startswith(HCL_PREFIX):
        return False
    code = val[len(HCL_PREFIX):]
    if len(code) != HCL_CODE_LEN:
        return False
    return all(is_valid_hcl_code_char(c) for c in code)


def is_valid_hcl_code_char(c):
    return '0' <= c <= '9' or 'a' <= c <= 'f'


VALID_ECL_VALUES = {'amb', 'blu', 'brn', 'gry', 'grn', 'hzl', 'oth'}


def ecl(val):
    return val in VALID_ECL_VALUES


PID_LEN = 9


def pid(val):
    if len(val) != PID_LEN:
        return False
    return all(c.isdigit() for c in val)


VALIDATORS = {
    'byr': byr,
    'iyr': iyr,
    'eyr': eyr,
    'hgt': hgt,
    'hcl': hcl,
    'ecl': ecl,
    'pid': pid
}


class ValidatorTests(unittest.TestCase):
    def test_byr(self):
        self.assertTrue(byr('2002'))
        self.assertFalse(byr('2003'))

    def test_hgt(self):
        self.assertTrue(hgt('60in'))
        self.assertTrue(hgt('190cm'))
        self.assertFalse(hgt('190in'))
        self.assertFalse(hgt('190'))

    def test_hcl(self):
        self.assertTrue(hcl('#123abc'))
        self.assertFalse(hcl('#123abz'))
        self.assertFalse(hcl('123abc'))

    def test_ecl(self):
        self.assertTrue(ecl('brn'))
        self.assertFalse(ecl('wat'))

    def test_pid(self):
        self.assertTrue(pid('000000001'))
        self.assertFalse(pid('0123456789'))

    def test_invalild_passports(self):
        text = '''eyr:1972 cid:100
        hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

        iyr:2019
        hcl:#602927 eyr:1967 hgt:170cm
        ecl:grn pid:012533040 byr:1946

        hcl:dab227 iyr:2012
        ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

        hgt:59cm ecl:zzz
        eyr:2038 hcl:74454a iyr:2023
        pid:3556412378 byr:2007'''

        passports = parse_passports(text)
        for passport in passports:
            self.assertFalse(is_valid_passport(VALIDATORS, passport))

    def test_valild_passports(self):
        text = '''pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
        hcl:#623a2f

        eyr:2029 ecl:blu cid:129 byr:1989
        iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

        hcl:#888785
        hgt:164cm byr:2001 iyr:2015 cid:88
        pid:545766238 ecl:hzl
        eyr:2022

        iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719'''

        passports = parse_passports(text)
        for passport in passports:
            self.assertTrue(is_valid_passport(VALIDATORS, passport))


if __name__ == '__main__':
    main()
