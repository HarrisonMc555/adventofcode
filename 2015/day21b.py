#!/usr/bin/env python3

INPUT_FILE = 'input21.txt'

import itertools

DEBUG = False
def debug_print(*args, **kwargs):
    if DEBUG:
        print(*args, **kwargs)

def main():
    if 'INPUT_FILE' not in globals():
        unittest.main()
        return
    boss_stats = parse_boss_lines(get_lines(INPUT_FILE))
    shop_info = parse_shop_info(SHOP_INFO_TEXT)
    print(get_least_gold(shop_info, boss_stats))

def get_least_gold(shop_info, boss_stats):
    combinations = get_combinations(shop_info)
    loses = lambda c: not is_winning_combination(boss_stats, c)
    losing_combinations = filter(loses, get_combinations(shop_info))
    return max(calc_price(c) for c in losing_combinations)

def is_winning_combination(boss_stats, combination):
    player_hit_points = 100
    player_damage = sum(d for _, d, _ in combination)
    player_armor = sum(a for _, _, a in combination)
    boss_hit_points, boss_damage, boss_armor = boss_stats

    player_to_boss_damage = calc_damage(player_damage, boss_armor)
    boss_to_player_damage = calc_damage(boss_damage, player_armor)
    while True:
        boss_hit_points -= player_to_boss_damage
        if boss_hit_points <= 0:
            return True
        player_hit_points -= boss_to_player_damage
        if player_hit_points <= 0:
            return False

def calc_damage(damage, armor):
    difference = damage - armor
    if difference <= 0:
        return 1
    else:
        return difference

def calc_price(combination):
    return sum(c for c, _, _ in combination)

def get_combinations(shop_info):
    weapons, armor, rings = shop_info
    debug_print(f'weapons: {weapons}')
    debug_print(f'armor: {armor}')
    debug_print(f'rings: {rings}')
    return list(list(flatten(t)) for t in itertools.product(
               get_weapons_options(weapons),
               get_armor_options(armor),
               get_rings_options(rings),
           ))

def flatten(iters):
    for outer in iters:
        for inner in outer:
            yield inner

def get_weapons_options(weapons):
    for w in weapons:
        yield [w]

def get_armor_options(armor):
    yield []
    for a in armor:
        yield [a]

def get_rings_options(rings):
    yield []
    for r in rings:
        yield [r]
    for rs in itertools.combinations(rings, 2):
        yield list(rs)

SHOP_INFO_TEXT = '''
Weapons:    Cost  Damage  Armor
Dagger        8     4       0
Shortsword   10     5       0
Warhammer    25     6       0
Longsword    40     7       0
Greataxe     74     8       0

Armor:      Cost  Damage  Armor
Leather      13     0       1
Chainmail    31     0       2
Splintmail   53     0       3
Bandedmail   75     0       4
Platemail   102     0       5

Rings:      Cost  Damage  Armor
Damage +1    25     1       0
Damage +2    50     2       0
Damage +3   100     3       0
Defense +1   20     0       1
Defense +2   40     0       2
Defense +3   80     0       3
'''

def parse_shop_info(text):
    blocks = text.split('\n\n')
    return tuple(parse_shop_block(block.strip()) for block in blocks)

def parse_shop_block(text):
    # Skip header
    lines = text.split('\n')[1:]
    return [parse_shop_line(line) for line in lines]

def parse_shop_line(line):
    return tuple(int(w) for w in line.split()[-3:])

def parse_boss_lines(lines):
    return [parse_boss_line(line) for line in lines]

def parse_boss_line(line):
    return int(line.split(' ')[-1])

def get_lines(filename):
    with open(filename) as f:
        return [line.strip() for line in f.readlines()]

if __name__ == '__main__':
    main()
