#!/usr/bin/env python3
with open('7.input', 'r') as f:
    data = f.read().splitlines()

rules = {}

for rule in data:
    # q - quantity
    # m - qualifier
    # c - color
    # kw1 - "bags"
    # kw2 - "contain"
    # kw3 - "bag[s]," | "bag[s]."
    # Valid forms:
    # m c "bags" "contain" (q m c "bag[s],")* q m c "bag[s]."
    # m c "bags" "contain" "no" "other" "bags."
    tokens = rule.split(' ')
    m0, c0, kw1, kw2 = tokens[:4]
    # Branch point
    br1 = tokens[4]
    if br1 == 'no':
        # Empty
        rules[(m0, c0)] = []
        continue
    # Parse multi-form formula
    contents = []
    for i in range(4, len(tokens), 4):
        q, m, c, kw3 = tokens[i:i+4]
        contents.append((int(q), m, c))
    rules[(m0, c0)] = contents

def task1():
    graph = { ('shiny', 'gold'): False }
    # Bag in question doesn't contain itself
    depth_max = 0
    depth = 0

    # Traverse the rules
    def traverse(color):
        nonlocal depth, depth_max
        depth += 1
        depth_max = max(depth_max, depth)
        if color == ('shiny', 'gold'):
            depth -= 1
            return True
        if color in graph:
            depth -= 1
            return graph[color]
        d = rules[color]
        for child in d:
            if traverse(child[1:3]):
                graph[color] = True
                depth -= 1
                return True
        graph[color] = False
        depth -= 1
        return False

    total = 0
    for color in rules:
        if color == ('shiny', 'gold'):
            continue
        if traverse(color):
            total += 1

    assert depth == 0
    print('The total is: {} with max depth: {}'.format(total, depth_max))

def task2():
    graph = {}
    depth_max = 0
    depth = 0

    # Traverse the rules & counts
    def traverse(color):
        nonlocal depth, depth_max
        depth += 1
        depth_max = max(depth_max, depth)
        if color in graph:
            depth -= 1
            return graph[color]
        d = rules[color]
        count = 0
        for child in d:
            count += child[0] * (1 + traverse(child[1:3]))
        graph[color] = count
        depth -= 1
        return count

    assert depth == 0
    print('Total bags contained is: {} with max depth: {}'.format(traverse(('shiny', 'gold')), depth_max))

task1()
task2()
