#!/usr/bin/env python3

REAL = False
REAL = True

import unittest
from collections import Counter, defaultdict
import itertools

DEBUG = False
# DEBUG = True
def debug_print(*args, **kwargs):
    if DEBUG:
        print(*args, **kwargs)

if REAL:
    POS_MIN_X = 94
    POS_MAX_X = 151
else:
    POS_MIN_X = 20
    POS_MAX_X = 30
BOUNDS_POS_X = POS_MIN_X, POS_MAX_X
RANGE_POS_X = range(POS_MIN_X, POS_MAX_X + 1)
if REAL:
    POS_MIN_Y = -156
    POS_MAX_Y = -103
else:
    POS_MIN_Y = -10
    POS_MAX_Y = -5
BOUNDS_POS_Y = POS_MIN_Y, POS_MAX_Y
RANGE_POS_Y = range(POS_MIN_Y, POS_MAX_Y + 1)
def main():
    print(run())
    if not REAL:
        unittest.main()

def run():
    num_steps_to_num_speed_init_x, num_steps_open_ended = \
        get_num_steps_to_num_speed_init_x(BOUNDS_POS_X)
    debug_print('Num steps -> Initial x speeds that are in range after that many steps')
    for num_steps, num_speed_init_x in num_steps_to_num_speed_init_x.items():
        debug_print(f'\t{num_steps} -> {num_speed_init_x}')
    debug_print('Num steps -> Initial x speeds that are in range FOREVER after that many steps')
    # debug_print(num_steps_open_ended)
    for num_steps, open_ended_speed_init_xs in num_steps_open_ended.items():
        debug_print(f'\t{num_steps} -> {open_ended_speed_init_xs}')
    num_steps_to_num_speed_init_y = get_num_steps_to_num_speed_init_y(BOUNDS_POS_Y)
    for num_steps, num_speed_init_y in num_steps_to_num_speed_init_y.items():
        debug_print(f'{num_steps} -> {num_speed_init_y}')

    pairs_set = set()
    for num_steps, speed_init_ys in num_steps_to_num_speed_init_y.items():
        for speed_init_y in speed_init_ys:
            for speed_init_x in num_steps_to_num_speed_init_x[num_steps]:
                pairs_set.add((speed_init_x, speed_init_y))
            for oes, open_ended_init_xs in num_steps_open_ended.items():
                if num_steps >= oes:
                    for speed_init_x in open_ended_init_xs:
                        pairs_set.add((speed_init_x, speed_init_y))
            
    return len(pairs_set)

def get_num_steps_to_num_speed_init_x(bounds_pos_x):
    pos_min_x, pos_max_x = bounds_pos_x
    num_steps_to_num_speed_init_x = defaultdict(list)
    num_steps_open_ended = defaultdict(list)
    for speed_init_x in range(pos_max_x + 1):
        cur_pos_x = 0
        cur_steps = 0
        speed_x = speed_init_x
        while cur_pos_x < pos_min_x:
            cur_steps += 1
            cur_pos_x += speed_x
            speed_x -= 1
            if speed_x <= 0:
                break
        if cur_pos_x < pos_min_x:
            continue
        steps_min = cur_steps
        while cur_pos_x <= pos_max_x:
            debug_print(f'\tspeed_init_x: {speed_init_x}, cur_steps: {cur_steps}, cur_pos_x: {cur_pos_x}')
            num_steps_to_num_speed_init_x[cur_steps].append(speed_init_x)
            
            cur_steps += 1
            cur_pos_x += speed_x
            speed_x -= 1
            if speed_x <= 0:
                num_steps_open_ended[cur_steps].append(speed_init_x)
                break
    return num_steps_to_num_speed_init_x, num_steps_open_ended

def get_num_steps_to_num_speed_init_y(bounds_pos_y):
    pos_min_y, pos_max_y = bounds_pos_y
    num_steps_to_num_speed_init_y = defaultdict(list)
    for speed_init_y in range(0, abs(pos_min_y) + 1):
        debug_print(f'speed_init_y: {speed_init_y}')
        cur_pos_y = 0
        cur_steps = 0
        speed_y = speed_init_y
        debug_print(f'\tcur_steps: {cur_steps}, cur_pos_y: {cur_pos_y}, speed_y: {speed_y}')
        debug_print('\tBefore range:')
        while cur_pos_y > pos_max_y:
            cur_steps += 1
            cur_pos_y += speed_y
            speed_y -= 1
            debug_print(f'\tcur_steps: {cur_steps}, cur_pos_y: {cur_pos_y}, speed_y: {speed_y}')
        steps_min = cur_steps
        debug_print('\tIn range:')
        while cur_pos_y >= pos_min_y:
            num_steps_to_num_speed_init_y[cur_steps].append(speed_init_y)
            cur_steps += 1
            cur_pos_y += speed_y
            speed_y -= 1
            debug_print(f'\tcur_steps: {cur_steps}, cur_pos_y: {cur_pos_y}, speed_y: {speed_y}')
        debug_print('\tOut of range')
    for speed_init_y in range(-1, pos_min_y - 1, -1):
        debug_print(f'speed_init_y: {speed_init_y}')
        cur_pos_y = 0
        cur_steps = 0
        speed_y = speed_init_y
        debug_print(f'\tcur_steps: {cur_steps}, cur_pos_y: {cur_pos_y}, speed_y: {speed_y}')
        debug_print('\tBefore range:')
        while cur_pos_y > pos_max_y:
            cur_steps += 1
            cur_pos_y += speed_y
            speed_y -= 1
            debug_print(f'\tcur_steps: {cur_steps}, cur_pos_y: {cur_pos_y}, speed_y: {speed_y}')
        steps_min = cur_steps
        debug_print('\tIn range:')
        while cur_pos_y >= pos_min_y:
            num_steps_to_num_speed_init_y[cur_steps].append(speed_init_y)
            cur_steps += 1
            cur_pos_y += speed_y
            speed_y -= 1
        debug_print('\tOut of range')
    return num_steps_to_num_speed_init_y

def sum_one_to_n(num):
    return num * (num + 1) // 2

def calc_dist_x(speed_init_x, num_steps):
    dist_max_x = sum_one_to_n(speed_init_x)
    if num_steps > speed_init_x:
        return dist_max_x
    else:
        return dist_max_x - sum_one_to_n(speed_init_x - num_steps)

class Test(unittest.TestCase):
    @unittest.skip
    def test_get_bounds_speed_x(self):
        bounds_speed_x = get_bounds_speed_x(BOUNDS_POS_X)
        speed_min_x, speed_max_x = bounds_speed_x
        for speed_x in range(speed_min_x, speed_max_x + 1):
            total_dist_x = sum_one_to_n(speed_x)
            self.assertIn(total_dist_x, RANGE_POS_X)
        for speed_x in [speed_min_x - 1, speed_max_x + 1]:
            total_dist_x = sum_one_to_n(speed_x)
            self.assertNotIn(total_dist_x, RANGE_POS_X)

    @unittest.skip
    def test_(self):
        speed_min_x = get_speed_min_x(BOUNDS_POS_X)
        speed_init_x_to_step_range = \
            get_num_steps_to_num_speed_init_x(BOUNDS_POS_X, speed_min_x)
        for speed_init_x, step_min_max in x2steps.items():
            steps_num_min, steps_num_max = step_min_max
            steps_range = range(steps_num_min, steps_num_max)
            self.assertNotIn(sum_one_to_n(speed_init_x))
            print(entry)

    def test_calc_dist_x(self):
        for speed_init_x in range(4):
            cur_dist = 0
            speed_x = speed_init_x
            for num_steps in range(4):
                calculated_dist = calc_dist_x(speed_init_x, num_steps)
                desc = f'speed_init_x: {speed_init_x}, num_steps:{num_steps}'
                self.assertEqual(calculated_dist, cur_dist, desc)
                if speed_x > 0:
                    cur_dist += speed_x
                    speed_x -= 1
                    
if __name__ == '__main__':
    main()
