#!/usr/bin/env python3

INPUT_FILE = 'input06.txt'
# INPUT_FILE = 'example.txt'

def main():
    with open(INPUT_FILE) as f:
        text = f.read().strip()
    groups = parse_response_groups(text)
    nums = [num_questions_answered(g) for g in groups]
    print(sum(nums))
    
def num_questions_answered(group):
    return len(questions_answered(group))

def questions_answered(group):
    questions = set()
    for response in group:
        questions.update(response)
    return questions

def parse_response_groups(text):
    groups = text.split('\n\n')
    return [parse_response_group(g) for g in groups]

def parse_response_group(text):
    return text.split('\n')

def debug_print_groups(groups):
    for i, group in enumerate(groups):
        print(f'Group {i}')
        for response in group:
            print(f'\t{response}')

if __name__ == '__main__':
    main()
