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

FIRST_JOB = 'A'
FIRST_JOB_ORD = ord(FIRST_JOB)
def job_length(value):
    return 60 + ord(value) - FIRST_JOB_ORD + 1

def create_job(value):
    return (value, job_length(value))

def job_done(job):
    _, time = job
    return time <= 0

def job_tick(job):
    value, time = job
    return (value, time - 1)

MAX_WORKERS = 5
def solve(dependency_list):
    forward_dependencies = build_forward_dependencies(dependency_list)
    reverse_dependencies = build_reverse_dependencies(forward_dependencies)
    no_dependencies = find_no_dependencies(reverse_dependencies)
    cur_no_dependencies = SortedCollection(no_dependencies, reverse=True)
    active_jobs = []
    for value in reversed(cur_no_dependencies[-MAX_WORKERS:]):
        # add as many jobs as you can
        # go in reverse order to go in order
        active_jobs.append(create_job(value))
        # delete assigned jobs
        del reverse_dependencies[value]
    for _ in range(len(cur_no_dependencies[-MAX_WORKERS:])):
        # delete assigned jobs
        cur_no_dependencies.pop()
    ticks = 0
    while active_jobs:
        active_jobs = [job_tick(job) for job in active_jobs]
        done_jobs, active_jobs = split_by(active_jobs, job_done)
        done_jobs = [value for value, _ in done_jobs]
        for dependencies in reverse_dependencies.values():
            # remove done jobs from dependencies
            dependencies.difference_update(done_jobs)
        for ready in find_no_dependencies(reverse_dependencies):
            # find new ready jobs and add them
            cur_no_dependencies.maybe_insert(ready)
            del reverse_dependencies[ready]
        while len(active_jobs) < MAX_WORKERS and cur_no_dependencies:
            # add as many jobs as you have workers and jobs for
            value = cur_no_dependencies.pop()
            active_jobs.append(create_job(value))
        ticks += 1
    # these should all be gone now...if not, there are some whose depencies were
    # never fulfilled (circular dependencies???)
    assert not reverse_dependencies
    return ticks

def split_by(enumerable, fun):
    trues, falses = [], []
    for value in enumerable:
        if fun(value):
            trues.append(value)
        else:
            falses.append(value)
    return trues, falses

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
