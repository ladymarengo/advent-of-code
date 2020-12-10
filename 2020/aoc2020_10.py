puzzle_input = open('aoc2020_10_input.txt').read().strip()

def main(text):
    adapters = sorted([int(line) for line in text.split('\n')])
    adapters.append(max(adapters)+3)
    adapters.insert(0, 0)

    diff1 = [(i) for i in range(len(adapters)-1) if adapters[i+1] == adapters[i]+1]
    diff3 = [(i) for i in range(len(adapters) - 1) if adapters[i + 1] == adapters[i] + 3]

    numbers_blocks = []
    first_index = 0
    for index in diff3:
        block = adapters[first_index:index+1]
        first_index = index+1
        numbers_blocks.append(block)
    numbers_blocks.append(adapters[diff3[-1]+1:])

    ways = 1
    for block in numbers_blocks:
        if len(block) == 3:
            ways = ways * 2
        elif len(block) == 4:
            ways = ways * 4
        elif len(block) == 5:
            ways = ways * 7

    print(len(diff1), 'differences of 1 jolt and', len(diff3), 'differences of 3 jolt')
    print(ways, 'ways for arranging the adapters')


test_input1 = '''16
10
15
5
1
11
7
19
6
12
4'''

test_input2 = '''28
33
18
42
31
14
46
20
48
47
24
23
49
45
19
38
39
11
1
32
25
35
8
17
7
9
4
2
34
10
3'''

main(test_input1)
main(test_input2)
main(puzzle_input)