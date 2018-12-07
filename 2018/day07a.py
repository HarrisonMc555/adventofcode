#!/usr/bin/env python3

import sys
import re
from collections import defaultdict
from SortedCollection import SortedCollection

def build_reverse_dependencies(forward_dependencies):
    reverse_dependencies = defaultdict(set)
    for dependence, dependents in forward_dependencies.items():
        # Ensure that the dependence ends up in the default dict, even if it
        # ends up with no dependencies
        _ = reverse_dependencies[dependence]
        for dependent in dependents:
            reverse_dependencies[dependent].add(dependence)
    return reverse_dependencies

def build_forward_dependencies(dependency_list):
    dependencies = defaultdict(set)
    for tup in dependency_list:
        dependence, dependent = tup
        dependencies[dependence].add(dependent)
    return dependencies

def find_no_dependencies(reverse_dependencies):
    return [v for v, dependencies in reverse_dependencies.items()
            if not dependencies]

def ordered_traversal(dependency_list):
    forward_dependencies = build_forward_dependencies(dependency_list)
    reverse_dependencies = build_reverse_dependencies(forward_dependencies)
    no_dependencies = find_no_dependencies(reverse_dependencies)
    cur_no_dependencies = SortedCollection(no_dependencies, reverse=True)
    answer = []
    while cur_no_dependencies:
        value = cur_no_dependencies.pop()
        del reverse_dependencies[value]
        answer.append(value)
        for dependencies in reverse_dependencies.values():
            dependencies.discard(value)
        for ready in find_no_dependencies(reverse_dependencies):
            cur_no_dependencies.maybe_insert(ready)
    return answer

def solve(dependency_list):
    return ''.join(ordered_traversal(dependency_list))

PATTERN = re.compile(r'Step (\w+) must be finished before step (\w+) can begin.')
def parse_dependency(line):
    return tuple(PATTERN.match(line).groups())

def get_input():
    return [parse_dependency(line.strip()) for line in sys.stdin.readlines()]

def main():
    dependency_list = get_input()
    print(solve(dependency_list))

if __name__ == '__main__':
    main()
