#!/usr/bin/env python3

INPUT_FILE = 'input01.txt'
SUM = 2020

def main():
    nums = get_nums(INPUT_FILE)
    sorted_nums = list(sorted(nums))
    x, y = find_pair_that_add_to(sorted_nums, SUM)
    product = x * y
    print(product)

def get_nums(input_file):
    with open(input_file) as f:
        return [int(line.strip()) for line in f.readlines()]

def get_nums_from_text(text):
    return [int(line.strip()) for line in f.readlines()]    

def find_pair_that_add_to(sorted_nums, total):
    index_low = 0
    index_high = len(sorted_nums) - 1
    while index_low < index_high:
        low = sorted_nums[index_low]
        high = sorted_nums[index_high]
        cur_total = low + high
        if cur_total < total:
            index_low += 1
        elif cur_total > total:
            index_high -= 1
        else:
            return (low, high)
    raise Exception(f'Did not find pair that add to {total}')

if __name__ == '__main__':
    main()
