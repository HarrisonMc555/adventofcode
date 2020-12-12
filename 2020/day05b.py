#!/usr/bin/env python3

import unittest
INPUT_FILE = 'input05.txt'

def main():
    # unittest.main()
    # return
    lines = get_lines_from_file(INPUT_FILE)
    seat_ids = [get_seat_id(line) for line in lines]
    print(get_missing_seat_id(seat_ids))

def get_missing_seat_id(seat_ids):
    seat_ids = set(seat_ids)
    cur_seat_id = min(seat_ids)
    last_seat_id = max(seat_ids)
    for seat_id in range(min(seat_ids), max(seat_ids)):
        if seat_id not in seat_ids:
            return seat_id
    raise Exception('No missing seat id')

def get_lines_from_file(filename):
    with open(filename) as f:
        return [line.strip() for line in f.readlines() if line]

SEAT_ID_ROW_MULTIPLIER = 8
def get_seat_id(text):
    row, column = get_seat(text)
    return row * SEAT_ID_ROW_MULTIPLIER + column

NUM_ROW_CHARS = 7
BASE = 2
def get_seat(text):
    row_chars = text[:NUM_ROW_CHARS]
    column_chars = text[NUM_ROW_CHARS:]

    row_chars = row_chars.replace('F', '0')
    row_chars = row_chars.replace('B', '1')
    column_chars = column_chars.replace('L', '0')
    column_chars = column_chars.replace('R', '1')

    row = int(row_chars, BASE)
    column = int(column_chars, BASE)

    return row, column

class ValidatorTests(unittest.TestCase):
    def test_seat(self):
        text = 'FBFBBFFRLR'
        row, column = get_seat(text)
        self.assertEqual(row, 44)
        self.assertEqual(column, 5)

    def test_seat_id(self):
        text = 'FBFBBFFRLR'
        seat_id = get_seat_id(text)
        self.assertEqual(seat_id, 357)

    def test_seat_ids(self):
        cases = [
            ('BFFFBBFRRR', 70, 7, 567),
            ('FFFBBBFRRR', 14, 7, 119),
            ('BBFFBBFRLL', 102, 4, 820),
        ]
        for text, row_exp, column_exp, seat_id_exp in cases:
            row_act, column_act = get_seat(text)
            self.assertEqual(row_exp, row_act)
            self.assertEqual(column_exp, column_act)
            seat_id_act = get_seat_id(text)
            self.assertEqual(seat_id_exp, seat_id_act)

if __name__ == '__main__':
    main()
