instructions = open('2016_1_input.txt').read()


def following_instructions(instr):
    directions_list = ['north', 'east', 'south', 'west']
    x, y = 0, 0
    direction = directions_list[0]
    visited_places = []

    def checking_visited_places(x, y):
        nonlocal visited_places
        if [x, y] not in visited_places:
            visited_places.append([x, y])
            return False
        else:
            return True

    for instruction in instr.split(', '):
        if instruction[0] == 'R':
            if directions_list.index(direction) < 3:
                direction = directions_list[directions_list.index(direction) + 1]
            else:
                direction = directions_list[0]
        if instruction[0] == 'L':
            if directions_list.index(direction) > 0:
                direction = directions_list[directions_list.index(direction) - 1]
            else:
                direction = directions_list[3]
        if direction == 'north':
            # y += int(instruction[1:])
            for i in range(int(instruction[1:])):
                y += 1
                if checking_visited_places(x, y):
                    return (abs(x) + abs(y))
        elif direction == 'east':
            # x += int(instruction[1:])
            for i in range(int(instruction[1:])):
                x += 1
                if checking_visited_places(x, y):
                    return (abs(x) + abs(y))
        elif direction == 'south':
            # y -= int(instruction[1:])
            for i in range(int(instruction[1:])):
                y -= 1
                if checking_visited_places(x, y):
                    return (abs(x) + abs(y))
        elif direction == 'west':
            # x -= int(instruction[1:])
            for i in range(int(instruction[1:])):
                x -= 1
                if checking_visited_places(x, y):
                    return (abs(x) + abs(y))
        # if [x, y] not in visited_places:
        #     visited_places.append([x, y])
        #     print(visited_places)
        # else:
        #     return (abs(x) + abs(y))
    # destination = abs(x) + abs(y)
    # return destination

if __name__ == '__main__':
     print(following_instructions(instructions))