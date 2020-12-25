#!/usr/bin/env python3

INPUT_FILE = 'input25.txt'
# INPUT_FILE = 'example25.txt'

MOD_NUM = 20201227
SUBJECT_NUM = 7

DEBUG = False

def main():
    text = get_text(INPUT_FILE)
    card_pk, door_pk = parse_text(text)
    debug_print(card_pk)
    debug_print(door_pk)
    card_loop_size, door_loop_size = extract_one_loop_size(card_pk, door_pk)
    # card_loop_size = extract_loop_size(card_pk)
    debug_print(card_loop_size)
    # door_loop_size = extract_loop_size(door_pk)
    debug_print(door_loop_size)
    if card_loop_size is not None:
        encryption_key = transform(door_pk, card_loop_size)
    elif door_loop_size is not None:
        encryption_key = transform(card_pk, door_loop_size)
    else:
        raise Exception('Both loop sizes were None!')
    # debug_print(encryption_key1)
    # debug_print(encryption_key2)
    # if encryption_key1 != encryption_key2:
    #     raise Exception('Encryption key 1 ({encryption_key1}) does not equal encryption key 2' + \
    #                     f'({encryption_key2})')
    print(encryption_key)
    
def transform(subject_num, loop_num):
    value = 1
    for _ in range(loop_num):
        value *= subject_num
        value %= MOD_NUM
    return value

def extract_loop_size(pk):
    value = 1
    count = 0
    while value != pk:
        debug_print(f'After {count} iterations, value is {value}')
        value *= SUBJECT_NUM
        value %= MOD_NUM
        count += 1
    return count

def extract_one_loop_size(card_pk, door_pk):
    card_value = 1
    door_value = 1
    count = 0
    while card_value != card_pk and door_value != door_pk:
        # debug_print(f'After {count} iterations, value is {value}')
        card_value *= SUBJECT_NUM
        door_value *= SUBJECT_NUM
        card_value %= MOD_NUM
        door_value %= MOD_NUM
        count += 1
    card_loop_size = count if card_value == card_pk else None
    door_loop_size = count if door_value == door_pk else None
    return card_loop_size, door_loop_size

def parse_text(text):
    card_pk, door_pk = [int(s) for s in text.split('\n')]
    return card_pk, door_pk

def get_text(filename):
    with open(filename) as f:
        return f.read().strip()

def debug_print(*args, **kwargs):
    if DEBUG:
        print(*args, **kwargs)

if __name__ == '__main__':
    main()
