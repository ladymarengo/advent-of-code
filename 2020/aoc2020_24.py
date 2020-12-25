puzzle_input = open('aoc2020_24_input.txt').read().strip()


def main(text):
    instructions = make_instructions(text)
    tiles = flipping_tiles(instructions)
    first_answer = len([x for x in tiles if tiles[x] == True])
    print('first answer is', first_answer)
    for i in range(100):
        tiles = day_flip(tiles)
    print('second_answer is', len([x for x in tiles if tiles[x] == True]))


def make_instructions(text):
    instructions = []
    for line in text.strip().split('\n'):
        instruction = []
        for i, symbol in enumerate(line):
            if (symbol == 'e' or symbol == 'w') and (i == 0 or (line[i-1] != 'n' and line[i-1] != 's')):
                instruction.append(symbol)
            elif symbol == 'n' or symbol == 's':
                instruction.append(symbol + line[i+1])
            else:
                continue
        instructions.append(instruction)
    return instructions


def flipping_tiles(instructions):
    tiles_dict = {}
    reference_tile = (0,0)
    tiles_dict[reference_tile] = False
    for instruction in instructions:
        x,y = 0,0
        for direction in instruction:
            if direction == 'e':
                x += 2
            elif direction == 'w':
                x -= 2
            elif direction == 'se':
                x += 1
                y += 1
            elif direction == 'sw':
                x -= 1
                y += 1
            elif direction == 'ne':
                x += 1
                y -= 1
            elif direction == 'nw':
                x -= 1
                y -= 1
        if (x,y) in tiles_dict and tiles_dict[(x,y)] == True:
            tiles_dict[(x, y)] = False
        else:
            tiles_dict[(x,y)] = True
    return tiles_dict


def day_flip(tiles):
    new_tiles = {}
    consider = []
    for tile in tiles:
        x = tile[0]
        y = tile[1]
        count = 0
        for neighbour in [(x-2,y),(x-1,y-1),(x+1,y-1),(x+2,y),(x+1,y+1),(x-1,y+1)]:
            if neighbour in tiles and tiles[neighbour] == True:
                count += 1
            elif neighbour not in tiles:
                consider.append(neighbour)
        if tiles[tile] == False and count == 2:
            new_tiles[tile] = True
        elif tiles[tile] == True and (count == 0 or count > 2):
            new_tiles[tile] = False
        else:
            new_tiles[tile] = tiles[tile]
    while len(consider) != 0:
        tile = consider.pop(0)
        x = tile[0]
        y = tile[1]
        count = 0
        for neighbour in [(x-2,y),(x-1,y-1),(x+1,y-1),(x+2,y),(x+1,y+1),(x-1,y+1)]:
            if neighbour in tiles and tiles[neighbour] == True:
                count += 1
        if count == 2:
            new_tiles[tile] = True
    return new_tiles


main(puzzle_input)