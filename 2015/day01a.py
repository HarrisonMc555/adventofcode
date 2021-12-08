#!/usr/bin/env python3

# TEST = True
TEST = False
INPUT_FILE = 'input01.txt'

def main():
    if TEST:
        test()
        return
    else:
        print(get_floor(get_text(INPUT_FILE)))

def get_floor(text):
    num_open = sum(1 for c in text if c == '(')
    num_closed = sum(1 for c in text if c == ')')
    floor = num_open - num_closed
    return floor

def get_text(filename):
    with open(filename) as f:
        return f.read().strip()

def test():
    test_cases = {
        '(())': 0,
        '()()': 0,
        '(((': 3,
        '(()(()(': 3,
        '))(((((': 3,
        '())': -1,
        '))(': -1,
        ')))': -3,
        ')())())': -3,
    }
    success = True
    for text, expected in test_cases.items():
        actual = get_floor(text)
        if actual != expected:
            success = False
            print(f'FAILED: got {actual}, expected {expected} from {text}')
    if success:
        print('SUCCESS')

if __name__ == '__main__':
    main()
