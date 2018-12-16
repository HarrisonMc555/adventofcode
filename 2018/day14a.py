#!/usr/bin/env python3
# pylint: disable=invalid-name

NUM_AFTER_DONE = 10
def solve(num_recipes):
    starting_recipes = [3, 7]
    answer_recipes = get_recipes_after(starting_recipes, num_recipes)
    return ''.join(str(recipe) for recipe in answer_recipes)

def get_recipes_after(recipes, num_recipes):
    first_index, second_index = 0, 1
    while len(recipes) < num_recipes + NUM_AFTER_DONE:
        first_index, second_index = combine(recipes, first_index, second_index)
    return recipes[num_recipes:num_recipes + NUM_AFTER_DONE]

def combine(recipes, first_index, second_index):
    first_score = recipes[first_index]
    second_score = recipes[second_index]
    new_recipes = get_digits(first_score + second_score)
    recipes.extend(new_recipes)
    new_first_index = (first_index + first_score + 1) % len(recipes)
    new_second_index = (second_index + second_score + 1) % len(recipes)
    return new_first_index, new_second_index

BASE = 10
def get_digits(num):
    assert num >= 0
    if num == 0:
        return [0]
    digits = []
    while num > 0:
        digits.append(num % BASE)
        num //= BASE
    return list(reversed(digits))

def get_input():
    return int(input())

def main():
    num_recipes = get_input()
    print(solve(num_recipes))

if __name__ == '__main__':
    main()
