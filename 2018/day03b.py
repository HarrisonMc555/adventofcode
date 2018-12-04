#!/usr/bin/env python3

import sys
import re
from collections import defaultdict

#pylint: disable=too-few-public-methods
class Claim:
    PATTERN = re.compile(r'#(\d+) @ (\d+),(\d+): (\d+)x(\d+)')
    #pylint: disable=too-many-arguments
    def __init__(self, claim_id, left, top, width, height):
        self.claim_id = claim_id
        self.left = left
        self.top = top
        self.width = width
        self.height = height

    @classmethod
    def from_string(cls, string):
        match = cls.PATTERN.match(string)
        nums = [int(w) for w in match.groups()]
        return Claim(*nums)

    def __str__(self):
        return '#{} @ {},{}: {}x{}'.format(self.claim_id, self.left, self.top,
                                           self.width, self.height)

def solve(claims):
    squares = defaultdict(list)
    for claim in claims:
        for square in claim_squares(claim):
            squares[square].append(claim.claim_id)
    valid_claim_ids = set(claim.claim_id for claim in claims)
    for square, claim_ids in squares.items():
        if len(claim_ids) > 1:
            valid_claim_ids.difference_update(claim_ids)
    assert len(valid_claim_ids) == 1
    return next(iter(valid_claim_ids))

def claim_squares(claim):
    return [(i, j)
            for i in range(claim.left, claim.left + claim.width)
            for j in range(claim.top, claim.top + claim.height)]

def get_input():
    return [Claim.from_string(line) for line in sys.stdin.readlines()]

def main():
    claims = get_input()
    print(solve(claims))

if __name__ == '__main__':
    main()
