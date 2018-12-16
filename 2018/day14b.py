#!/usr/bin/env python3
# pylint: disable=invalid-name

NUM_AFTER_DONE = 10
def solve(pattern):
    starting_recipes = [3, 7]
    return get_num_recipes_before(starting_recipes, pattern)

def get_num_recipes_before(recipes, pattern):
    recipe_index = 0
    num_recipes = len(pattern)
    first_index, second_index = 0, 1
    while not pattern_matches(recipes, recipe_index, pattern):
        recipe_index += 1
        while len(recipes) < recipe_index + num_recipes:
            first_index, second_index = combine(recipes, first_index, second_index)
    return recipe_index

def pattern_matches(recipes, recipe_index, pattern):
    return recipes[recipe_index:recipe_index + len(pattern)] == pattern

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
    return [int(x) for x in input().strip()]

def main():
    pattern = get_input()
    print(solve(pattern))

if __name__ == '__main__':
    main()
