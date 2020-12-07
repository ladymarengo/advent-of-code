import re


def main(text, search_item):
    answers = []

    def create_graph(text):
        graph = {}
        numbers_graph = {}
        rules = text.split('\n')
        for rule in rules:
            result = re.search(r"([a-z]+ [a-z]+) bags contain (.+).", rule)
            bag = result.group(1)
            contain = result.group(2)
            graph[bag] = []
            numbers_graph[bag] = []
            if contain != 'no other bags':
                for item in contain.split(', '):
                    res = re.search(r"([0-9]+) (.+) bag.*", item)
                    graph[bag].append(res.group(2))
                    numbers = [int(res.group(1)), res.group(2)]
                    numbers_graph[bag].append(numbers)
        return graph, numbers_graph

    def check_container(graph, parents, item, search_item):
        nonlocal answers
        if search_item in graph[item]:
            answers += parents
            return True
        elif len(graph[item]) == 0:
            return None
        for bag in graph[item]:
            new_parents = parents.copy()
            new_parents.append(bag)
            check_container(graph, new_parents, bag, search_item)

    def check_amount(graph, item):
        if len(graph[item]) == 0:
            return 0
        amount = 0
        for bag in graph[item]:
            amount += bag[0] * (1 + check_amount(graph, bag[1]))
        return amount

    bags_dict, numbers_dict = create_graph(text)

    for item in bags_dict:
        parents = []
        parents.append(item)
        check_container(bags_dict, parents, item, search_item)

    amount = check_amount(numbers_dict, search_item)
    print('answer for first puzzle is ', len(set(answers)))
    print('answer for second puzzle is ', amount)


puzzle_input = open('aoc2020_7_input.txt').read().strip()

test_input = '''light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.'''

test2_input = '''shiny gold bags contain 2 dark red bags.
dark red bags contain 2 dark orange bags.
dark orange bags contain 2 dark yellow bags.
dark yellow bags contain 2 dark green bags.
dark green bags contain 2 dark blue bags.
dark blue bags contain 2 dark violet bags.
dark violet bags contain no other bags.'''

main(test_input, 'shiny gold')
main(test2_input, 'shiny gold')
main(puzzle_input, 'shiny gold')