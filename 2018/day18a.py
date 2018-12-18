#!/usr/bin/env python3
#pylint: disable=invalid-name, too-few-public-methods

import sys
import re
from enum import Enum, auto

################################################################################
# Run
################################################################################
def main():
    grid = get_input()
    print(solve(grid))

if __name__ == '__main__':
    main()
