puzzle_input = open('aoc2020_6_input.txt').read().strip()

test_input = '''abc

a
b
c

ab
ac

a
a
a
a

b'''

def first(text):
    groups = text.split('\n\n')
    all_answers = 0
    for group in groups:
        group = ''.join(group.split())
        answers = []
        for answer in group:
            if answer not in answers:
                answers.append(answer)
        all_answers += len(answers)
    print(all_answers)

def second(text):
    groups = text.split('\n\n')
    all_answers = 0
    for group in groups:
        people = len(group.split('\n'))
        group = ''.join(group.split())
        answers = {}
        for answer in group:
            if answer not in answers:
                answers[answer] = 1
            else:
                answers[answer] += 1
        for key in answers:
            if answers[key] == people:
                all_answers += 1
    print(all_answers)

first(test_input)
first(puzzle_input)
second(test_input)
second(puzzle_input)