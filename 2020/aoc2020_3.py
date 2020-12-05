puzzle_input = open('aoc2020_3_input.txt').read().strip().split('\n')
test_input = '''..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#'''

def main(map):
    trees1 = slope(map, 3, 1)
    trees2 = slope(map, 1, 1)
    trees3 = slope(map, 5, 1)
    trees4 = slope(map, 7, 1)
    trees5 = slope(map, 1, 2)
    print(trees2 * trees1 * trees3 * trees4 * trees5)

def slope(map, x_chande, y_change):
    x = 0
    y = 0
    trees = 0
    map_width = len(map[0])
    for step in range(len(map) - 1):
        x += x_chande
        y += y_change
        x_map = x % map_width
        if y < len(map) and map[y][x_map] == '#':
            trees += 1
    return trees

main(test_input.strip().split('\n'))
main(puzzle_input)