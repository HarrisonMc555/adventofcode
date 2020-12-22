#!/usr/bin/env python3

import re

INPUT_FILE = 'input21.txt'
# INPUT_FILE = 'example21.txt'

def main():
    text = get_text(INPUT_FILE)
    ingredients_list = parse_text(text)
    # for ingredients, allergens in ingredients_list:
    #     print(f'Ingredients: {ingredients}, allergens: {allergens}')
    # print()
    allergen_to_ingredient = match_allergen_to_ingredient(ingredients_list)
    ingredient_to_allergen = {i: a for a, i in allergen_to_ingredient.items()}
    ingredients = list(ingredient_to_allergen.keys())
    ingredients.sort(key=lambda i: ingredient_to_allergen[i])
    print(','.join(ingredients))

def match_allergen_to_ingredient(ingredients_list):
    allergens_to_possible_ingredients = find_allergen_possible_ingredients(ingredients_list)
    return match_allergen_to_ingredient_helper(allergens_to_possible_ingredients, {})
    # allergens = set(a for _, allergens in ingredients_list for a in allergens)
    # ingredients = set(i for ingredients, _ in ingredients_list for i in ingredients)

def match_allergen_to_ingredient_helper(allergens_to_possible_ingredients, cur_map):
    if not allergens_to_possible_ingredients:
        return cur_map
    allergens = list(allergens_to_possible_ingredients)
    allergens.sort(key=lambda a: len(allergens_to_possible_ingredients[a]))
    allergen = allergens[0]
    possible_ingredients = allergens_to_possible_ingredients.pop(allergen)
    for ingredient in possible_ingredients:
        new_map = dict(cur_map)
        new_map[allergen] = ingredient
        new_allergens_to_possible_ingredients = dict(allergens_to_possible_ingredients)
        for ingredients_set in new_allergens_to_possible_ingredients.values():
            ingredients_set.discard(ingredient)
        result = match_allergen_to_ingredient_helper(new_allergens_to_possible_ingredients, new_map)
        if result:
            return result
    # Nothing worked
    return None

def find_allergen_possible_ingredients(ingredients_list):
    allergens = set(a for _, allergens in ingredients_list for a in allergens)
    return {a: find_possible_inredients_for_allergen(a, ingredients_list) for a in allergens}

def find_possible_inredients_for_allergen(allergen, ingredients_list):
    possible_ingredients_sets = [in_set for in_set, a_set in ingredients_list
                                 if allergen in a_set]
    result = set(possible_ingredients_sets[0])
    for in_set in possible_ingredients_sets[1:]:
        result.intersection_update(in_set)
    return result

def find_ingredients_that_cannot_have_allergens(ingredients_list):
    all_ingredients = {i for ingredients, _ in ingredients_list for i in ingredients}
    # print(f'all_ingredients: {all_ingredients}')
    allergens_to_possible_ingredients = find_allergen_possible_ingredients(ingredients_list)
    # print(f'allergens_to_possible_ingredients:')
    # for allergen, possible_ingredients in allergens_to_possible_ingredients.items():
    #     print(f'\t{allergen} => {possible_ingredients}')
    ingredients_with_possible_allergens = {i for il in allergens_to_possible_ingredients.values()
                                           for i in il} 
    # print(f'ingredients_with_possible_allergens: {ingredients_with_possible_allergens}')
    ingredients_without_allergens = all_ingredients.difference(ingredients_with_possible_allergens)
    # print(f'ingredients_without_allergens: {ingredients_without_allergens}')
    return ingredients_without_allergens

def count_apperance_of_ingredient(ingredient, ingredients_list):
    # print(f'count_apperance_of_ingredient({ingredient}, [...])')
    # for ingredients, allergens in ingredients_list:
    #     print(f'Ingredients: {ingredients}, allergens: {allergens}')
    # print(f'\t{[ingredient in in_set for in_set, _ in ingredients_list]}')
    # print()
    return [ingredient in in_set for in_set, _ in ingredients_list].count(True)

def get_text(filename):
    with open(filename) as f:
        return f.read().strip()

def parse_text(text):
    lines = text.split('\n')
    return [parse_line(line) for line in lines]

LINE_RE = re.compile(r'^([a-z ]+) \(contains ([a-z ,]+)\)$')
def parse_line(line):
    match = LINE_RE.match(line)
    if not match:
        print(f'Line \'{line}\' did not match')
    ingredients_text, allergens_text = match.groups()
    ingredients = set(ingredients_text.split(' '))
    allergens = set(allergens_text.split(', '))
    return ingredients, allergens

if __name__ == '__main__':
    main()
