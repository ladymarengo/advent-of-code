import re

puzzle_input = open('aoc2020_2_input.txt').read().strip().split('\n')
test_input = ['1-3 a: abcde', '1-3 b: cdefg', '2-9 c: ccccccccc']

def first_puzzle(passwords):
    valid_passwords = 0
    for line in passwords:
        res = re.search(r"([0-9]+)-([0-9]+)\s*([a-z]+):\s*([a-z]+)", line)
        amount = re.findall(res.group(3), res.group(4))
        if len(amount) >= int(res.group(1)) and len(amount) <= int(res.group(2)):
            valid_passwords += 1
    print(valid_passwords)

def second_puzzle(passwords):
    valid_passwords = 0
    for line in passwords:
        res = re.search(r"([0-9]+)-([0-9]+)\s*([a-z]+):\s*([a-z]+)", line)
        password = res.group(4)
        symbol = res.group(3)
        int1 = int(res.group(1)) - 1
        int2 = int(res.group(2)) - 1
        if (symbol == password[int1]) ^ (symbol == password[int2]):
            valid_passwords += 1
    print(valid_passwords)

first_puzzle(test_input)
first_puzzle(puzzle_input)
second_puzzle(test_input)
second_puzzle(puzzle_input)