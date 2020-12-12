puzzle_input = open('aoc2020_12_input.txt').read().strip()

def first_puzzle(text):
    instructions = text.split('\n')
    directions = {0:'N', 90:'E', 180:'S', 270:'W'}
    actions = ['N', 'E', 'S', 'W']
    degrees = 90
    x, y = 0, 0

    def execute(action, value):
        nonlocal x, y
        if action == 'N':
            y += value
        elif action == 'E':
            x += value
        elif action == 'S':
            y -= value
        elif action == 'W':
            x -= value

    for instruction in instructions:
        action = instruction[0]
        value = int(instruction[1:])
        if action in actions:
            execute(action, value)
            continue
        elif action == 'F':
            execute(directions[degrees], value)
        elif action == 'L':
            degrees -= value
        elif action == 'R':
            degrees += value
        degrees = degrees%360

    distance = abs(x) + abs(y)
    print(distance)

def second_puzzle(text):
    instructions = text.split('\n')
    x, y = 0, 0
    waypoint = [['N', 1], ['E', 10], ['S', 0], ['W', 0]]

    def execute(action, value):
        nonlocal x, y
        if action == 'N':
            y += value
        elif action == 'E':
            x += value
        elif action == 'S':
            y -= value
        elif action == 'W':
            x -= value
        return x, y

    def rotate_waypoint(action, value):
        nonlocal waypoint
        turn = value/90
        if action == 'L':
            turn = 0 - turn
        new_list = [['N', 0], ['E', 0], ['S', 0], ['W', 0]]
        for i, direction in enumerate(waypoint):
            new_index = int((i+turn)%len(waypoint))
            new_list[new_index][1] = waypoint[i][1]
        waypoint = new_list

    for instruction in instructions:
        action = instruction[0]
        value = int(instruction[1:])
        if action == 'F':
            for direction in waypoint:
                execute(direction[0], value*direction[1])
        elif action in ['L', 'R']:
            rotate_waypoint(action, value)
        else:
            for dir in waypoint:
                if dir[0] == action:
                    dir[1] += value

    distance = abs(x) + abs(y)
    print(distance)

test_input = '''F10
N3
F7
R90
F11'''

first_puzzle(test_input)
first_puzzle(puzzle_input)
second_puzzle(test_input)
second_puzzle(puzzle_input)