puzzle_input = open('aoc2020_1_input.txt').read().strip().split('\n')

test_input = ['1721', '979', '366', '299', '675', '1456']

for i, first_entry in enumerate(puzzle_input):
    for j, second_entry in enumerate(puzzle_input):
        for k, third_entry in enumerate(puzzle_input):
            if i != j and i != k and j != k and int(first_entry) + int(second_entry) + int(third_entry) == 2020:
                print(first_entry, second_entry, third_entry, int(first_entry)*int(second_entry)*int(third_entry))