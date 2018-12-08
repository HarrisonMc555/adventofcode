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

def postorder_metadata(root):
    children, metadata = root
    answer = []
    for child in children:
        answer.extend(postorder_metadata(child))
    answer.extend(metadata)
    return answer

def solve(nums):
    root = parse_tree(nums)
    metadata = postorder_metadata(root)
    return sum(metadata)

def get_input():
    return [int(w) for w in input().split(' ')]

def main():
    nums = get_input()
    print(solve(nums))

if __name__ == '__main__':
    main()
