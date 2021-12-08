#!/usr/bin/env python3

# TEST = True
TEST = False
INPUT_FILE = 'input01.txt'

def main():
    if TEST:
        test()
        return
    else:
        print(get_index_for_basement(get_text(INPUT_FILE)))

def get_index_for_basement(text):
    floor = 0
    for i, c in enumerate(text):
        if c == '(':
            floor += 1
        elif c == ')':
            floor -= 1
        else:
            raise Exception(f'Unexpected character {c}')
        if floor == -1:
            return i + 1
    raise Exception('Never enetered basement')

def get_text(filename):
    with open(filename) as f:
        return f.read().strip()

def test():
    test_cases = {
        ')': 1,
        '()())': 5,
    }
    success = True
    for text, expected in test_cases.items():
        actual = get_index_for_basement(text)
        if actual != expected:
            success = False
            print(f'FAILED: got {actual}, expected {expected} from {text}')
    if success:
        print('SUCCESS')

if __name__ == '__main__':
    main()
