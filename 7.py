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

    # Traverse the rules
    def traverse(color):
        if color == ('shiny', 'gold'):
            return True
        if color in graph:
            return graph[color]
        d = rules[color]
        for child in d:
            if traverse(child[1:3]):
                graph[color] = True
                return True
        graph[color] = False
        return False

    total = 0
    for color in rules:
        if color == ('shiny', 'gold'):
            continue
        if traverse(color):
            total += 1

    print('The total is: {}'.format(total))

def task2():
    graph = {}

    # Traverse the rules & counts
    def traverse(color):
        if color in graph:
            return graph[color]
        d = rules[color]
        count = 0
        for child in d:
            count += child[0] * (1 + traverse(child[1:3]))
        graph[color] = count
        return count

    print('Total bags contained is: {}'.format(traverse(('shiny', 'gold'))))

task2()
