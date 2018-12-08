#!/usr/bin/env python3

def parse_node(nums):
    num_children, num_metadata = nums[0:2]
    nums = nums[2:]
    children = []
    for _ in range(num_children):
        child, nums = parse_node(nums)
        children.append(child)
    metadata, nums = nums[:num_metadata], nums[num_metadata:]
    node = children, metadata
    return node, nums

def parse_tree(nums):
    root, rest = parse_node(nums)
    assert not rest
    return root

def node_value(node):
    children, metadata = node
    if not children:
        return sum(metadata)
    value = 0
    for index in metadata:
        if 1 <= index <= len(children):
            value += node_value(children[index - 1])
    return value

def solve(nums):
    root = parse_tree(nums)
    return node_value(root)

def get_input():
    return [int(w) for w in input().split(' ')]

def main():
    nums = get_input()
    print(solve(nums))

if __name__ == '__main__':
    main()
