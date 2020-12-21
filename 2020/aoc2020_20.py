import re
from math import sqrt


puzzle_input = open('aoc2020_20_input.txt').read().strip()


def main(text):
    tiles = make_tiles(text)
    borders, corner_tiles, border_tiles, inner_tiles = count_tiles(tiles)
    first_answer = 1
    for number in corner_tiles:
        first_answer = first_answer * int(number.name)
    print('answer for first puzzle is', first_answer)
    image = construct_image(borders, corner_tiles, border_tiles, inner_tiles)
    final_image = adjust_image(image)
    states = get_states(final_image)
    amount, state = find_sea_monsters(states)
    second_answer = count_monsters(state, amount)
    print('answer for second puzzle is', second_answer)


class TileOrientation:
    def __init__(self, name, up, right, down, left, tile):
        self.name = name
        self.up = up
        self.right = right
        self.down = down
        self.left = left
        self.tile = tile

    def __repr__(self):
        return f"Tile({self.name})"


class Tile:
    def __init__(self, name, up, right, down, left, tile):
        self.name = name
        self.up = up
        self.right = right
        self.down = down
        self.left = left
        self.tile = tile

    def __eq__(self, other):
        return self.name == other.name

    def __hash__(self):
        return hash(self.name)

    def __repr__(self):
        return f"Tile({self.name})"

    def get_all_orientations(self):
        result = []
        result.append(TileOrientation(self.name, self.up, self.right, self.down, self.left, self.tile))
        new_up, new_right, new_down, new_left, new_tile = self.up, self.right, self.down, self.left, self.tile

        def rotate(up, right, down, left, tile):
            new_right, new_down, new_left, new_up = up, right, down, left
            new_tile = []
            for i in range(len(tile[0])):
                new_line = []
                for j in range(len(tile[0]) - 1, -1, -1):
                    new_line.append(tile[j][i])
                new_tile.append(''.join(new_line))
            return  new_up, new_right, new_down, new_left, new_tile

        def flip(up, right, down, left, tile):
            reversed_up, reversed_down = up, down
            reversed_up.value, reversed_down.value = up.value[::-1], down.value[::-1]
            reversed_right, reversed_left = left, right
            reversed_tile = []
            for line in self.tile:
                reversed_tile.append(line[::-1])
            return reversed_up, reversed_right, reversed_down, reversed_left, reversed_tile

        for n in range(3):
            new_up, new_right, new_down, new_left, new_tile = rotate(new_up, new_right, new_down, new_left, new_tile)
            result.append(TileOrientation(self.name, new_up, new_right, new_down, new_left, new_tile))

        new_up, new_right, new_down, new_left, new_tile = flip(self.up, self.right, self.down, self.left, self.tile)
        result.append(TileOrientation(self.name, new_up, new_right, new_down, new_left, new_tile))

        for n in range(3):
            new_up, new_right, new_down, new_left, new_tile = rotate(new_up, new_right, new_down, new_left, new_tile)
            result.append(TileOrientation(self.name, new_up, new_right, new_down, new_left, new_tile))

        return result


class Side:
    def __init__(self, value, unique=False):
        self.value = value
        self.unique = unique

    def __eq__(self, other):
        return self.value == other.value

    def __hash__(self):
        return hash(self.value)

    def __repr__(self):
        return f"Side({self.value})"


def make_tiles(text):
    tiles_list = []
    for block in text.split('\n\n'):
        result = re.search(r'e ([0-9]+):\n([\S\s]+)', block)
        key = result.group(1)
        tile = result.group(2).split('\n')
        border1, border3 = tile[0], tile[-1]
        border2, border4 = [], []
        for line in tile:
            border4.append(line[0])
            border2.append(line[-1])
        tiles_list.append(Tile(key, Side(border1), Side(''.join(border2)), Side(border3[::-1]), Side(''.join(border4)[::-1]), tile))
    return tiles_list


def count_tiles(tiles):
    borders_list = []
    corner_tiles = []
    border_tiles = []
    inner_tiles = []
    for tile in tiles:
        borders_list += [tile.up.value, tile.right.value, tile.down.value, tile.left.value]
    for tile in tiles:
        count_borders = 0
        for border in [tile.up, tile.right, tile.down, tile.left]:
            count = 0
            count += borders_list.count(border.value)
            count += borders_list.count(border.value[::-1])
            if count == 1:
                count_borders += 1
                border.unique = True
        if count_borders == 2:
            corner_tiles.append(tile)
        elif count_borders == 1:
            border_tiles.append(tile)
        else:
            inner_tiles.append(tile)
    return borders_list, corner_tiles, border_tiles, inner_tiles


def construct_image(borders, corner_tiles, border_tiles, inner_tiles):
    length = int(sqrt(len(borders)/4))
    image = []

    def find_tile(tiles_list, first_condition=None, second_condition=None,
                  left_neighbour=None, up_neighbour=None):
        for tile in tiles_list:
            for to in tile.get_all_orientations():
                if (first_condition is None or getattr(to, first_condition).unique) \
                        and (second_condition is None or getattr(to, second_condition).unique) \
                        and (left_neighbour is None or left_neighbour.right.value == to.left.value[::-1] or left_neighbour.right.value == to.left.value) \
                        and (up_neighbour is None or up_neighbour.down.value == to.up.value[::-1] or up_neighbour.down.value == to.up.value):
                    tiles_list.remove(tile)
                    return to

    #first corner
    image.append([find_tile(corner_tiles, 'left', 'up')])
    #up line
    for i in range(1, length-1):
        image[0].append(find_tile(border_tiles, 'up', left_neighbour=image[0][i-1]))
    #second corner
    image[0].append(find_tile(corner_tiles, 'right', 'up', image[0][-1]))
    #inner lines
    for i in range(1, length - 1):
        image.append([find_tile(border_tiles, 'left', up_neighbour=image[i-1][0])])
        for j in range(1, length - 1):
            image[i].append(find_tile(inner_tiles, left_neighbour=image[i][j - 1], up_neighbour=image[i-1][j]))
        image[i].append(find_tile(border_tiles, 'right', left_neighbour=image[i][-1], up_neighbour=image[i-1][-1]))
    #last line
    image.append([find_tile(corner_tiles, 'left', 'down', up_neighbour=image[-1][0])])
    for i in range(1, length-1):
        image[-1].append(find_tile(border_tiles, 'down', left_neighbour=image[-1][i-1], up_neighbour=image[-2][i]))
    image[-1].append(find_tile(corner_tiles, 'right', 'down', left_neighbour=image[-1][-1], up_neighbour=image[-2][-1]))

    return image


def adjust_image(image):

    def adjust_tile(tile):
        new_tile = []
        for i in range(1, len(tile)-1):
            new_tile.append(tile[i][1:-1])
        return new_tile

    new_image = []
    for line in image:
        new_line = []
        for tile in line:
            new_line.append(adjust_tile(tile.tile))
        new_image.append(new_line)

    def remove_gaps(line):
        new_lines = []
        for i in range(len(line[0])):
            new_line = ''
            for tile in line:
                new_line += tile[i]
            new_lines.append(new_line)
        return new_lines

    adjusted_image = []
    for i in range(len(new_image)):
        adjusted_image = adjusted_image + remove_gaps(new_image[i])
    return adjusted_image


def get_states(image):

    def rotate(image):
        new_image = []
        for i in range(len(image)):
            new_line = []
            for j in range(len(image) -1, -1, -1):
                new_line.append(image[j][i])
            new_image.append(''.join(new_line))
        return new_image

    def flip(image):
        reversed_image = []
        for line in image:
            reversed_image.append(line[::-1])
        return reversed_image

    states = []
    states.append(image)
    for i in range(3):
        image = rotate(image)
        states.append(image)
    image = flip(image)
    states.append(image)
    for i in range(3):
        image = rotate(image)
        states.append(image)
    return states


def find_sea_monsters(image_states):
    sea_monster = '''                  # 
#    ##    ##    ###
 #  #  #  #  #  #'''

    def find_in_state(image, monster):
        count = 0
        for i in range(len(image)-len(monster)+1):
            for j in range(len(image[i])-len(monster[1])+1):
                if check_monster(image, i, j, monster):
                    count += 1
        return count

    def check_monster(image, i, j, monster):
        for m, line in enumerate(monster):
            for l, symbol in enumerate(line):
                if symbol == '#' and image[i + m][j + l] != '#':
                    return False
        return True

    results = {}
    for state in image_states:
        results[find_in_state(state, sea_monster.split('\n'))] = state
    return max(results), results[max(results)]


def count_monsters(image, amount):
    monsters_symbols = amount * 15
    symbols_in_image = len([x for line in image for x in line if x == '#'])
    return symbols_in_image-monsters_symbols


main(puzzle_input)
