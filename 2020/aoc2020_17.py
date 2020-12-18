puzzle_input = open('aoc2020_17_input.txt').read().strip()


def main(initial_state):
    active = analyse_input(initial_state)
    for i in range(6):
        active = cycle(active)
    print(len(active))


def analyse_input(state):
    active = set()
    for i, line in enumerate(state):
        for j, symbol in enumerate(line):
            if symbol == '#':
                active.add((j,i,0,0))
    return active


def find_neighbours(cube):
    neighbours = [(x,y,z,w) for x in range(cube[0]-1, cube[0]+2) for y in range(cube[1]-1, cube[1]+2) for z in range(cube[2]-1, cube[2]+2) for w in range(cube[3]-1, cube[3]+2) if (x,y,z,w) != cube]
    return neighbours


def cycle(active):
    remain_active = set()
    consider = set()
    for cube in active:
        active_neighbours = 0
        for neighbour in find_neighbours(cube):
            if neighbour in active:
                active_neighbours += 1
            else:
                consider.add(neighbour)
        if active_neighbours == 2 or active_neighbours == 3:
            remain_active.add(cube)

    for cube in consider:
        active_neighbours = 0
        for neighbour in find_neighbours(cube):
            if neighbour in active:
                active_neighbours += 1
        if active_neighbours == 3:
            remain_active.add(cube)
    return remain_active


main(puzzle_input.split('\n'))