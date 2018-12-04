#!/usr/bin/env python3

import sys
import re
from collections import Counter

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
    claimed = Counter()
    for claim in claims:
        claimed.update(claim_squares(claim))
    have_multiple_claims = [square for square, num in claimed.items()
                            if num > 1]
    return len(have_multiple_claims)

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
