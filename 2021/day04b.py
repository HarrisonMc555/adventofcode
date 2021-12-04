#!/usr/bin/env python3

INPUT_FILE = 'input04.txt'
# INPUT_FILE = 'example04.txt'

def main():
    text = get_text(INPUT_FILE)
    nums, boards = parse_text(text)
    unmarked_nums, num = run_game(nums, boards)
    print(sum(unmarked_nums) * num)

def run_game(nums, boards):
    mark_boards = [create_mark_board() for _ in boards]
    marked_nums = set()
    for num in nums:
        marked_nums.add(num)
        new_boards = [board for board in boards if not won(board, marked_nums)]
        if not new_boards:
            unmarked_nums = get_unmarked_nums(boards[0], marked_nums)
            return unmarked_nums, num
        boards = new_boards
    else:
        raise Exception('Some boards ever won')

def won(board, marked_nums):
    try:
        for col_i in range(5):
            if all(row[col_i] in marked_nums for row in board):
                return True
        for row_i in range(5):
            if all(board[row_i][col_i] in marked_nums for col_i in range(5)):
                return True
        return False
    except Exception as e:
        print(f'Problem with board: {board} with marked_nums: {marked_nums}')
        raise e

def get_unmarked_nums(board, marked_nums):
    return [num for row in board for num in row if num not in marked_nums]

def create_mark_board():
    return [[False]*5 for _ in range(5)]

def parse_text(text):
    groups = text.split('\n\n')
    nums_line = groups[0]
    nums = [int(n) for n in nums_line.split(',')]
    board_groups = groups[1:]
    boards = [parse_board(group) for group in board_groups]
    return nums, boards

def parse_board(text):
    return [[int(n) for n in line.split()] for line in text.split('\n') if line]

def get_text(filename):
    with open(filename) as f:
        return f.read()

if __name__ == '__main__':
    main()
