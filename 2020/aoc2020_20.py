import re
from math import sqrt


puzzle_input = open('aoc2020_20_input.txt').read().strip()
test_input = open('aoc2020_20_test_input.txt').read().strip()


def main(text):
    tiles = make_tiles(text)
    borders, corner_tiles, border_tiles, inner_tiles = count_tiles(tiles)
    first_answer = 1
    for number in corner_tiles:
        first_answer = first_answer * int(number)
    print('answer for first puzzle is ', first_answer)


class Tile():
    def __init__(self, name, up, right, down, left):
        self.name = name
        self.up = up
        self.right = right
        self.down = down
        self.left = left

    def __eq__(self, other):
        return self.name == other.name

    def __hash__(self):
        return hash(self.name)

    def __repr__(self):
        return f"Field({self.name})"


def make_tiles(text):
    tiles_dict = {}
    for block in text.split('\n\n'):
        result = re.search(r'e ([0-9]+):\n([\S\s]+)', block)
        key = result.group(1)
        tile = result.group(2).split('\n')
        border1, border3 = tile[0], tile[-1]
        border2, border4 = [], []
        for line in tile:
            border4.append(line[0])
            border2.append(line[-1])
        tiles_dict[key] = [border1, ''.join(border2), border3[::-1], ''.join(border4)[::-1]]
    return tiles_dict


def count_tiles(tiles):
    borders_list = []
    corner_tiles = []
    border_tiles = []
    inner_tiles = []
    for tile in tiles:
        borders_list += tiles[tile]
    for tile in tiles:
        count_borders = 0
        for border in tiles[tile]:
            count = 0
            count += borders_list.count(border)
            count += borders_list.count(border[::-1])
            if count == 1:
                count_borders += 1
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


main(test_input)
main(puzzle_input)