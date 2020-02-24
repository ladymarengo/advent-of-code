from functools import reduce


def splitting(string):
    x1y1 = (string.split()[-3]).split(',')
    x2y2 = (string.split()[-1]).split(',')
    return [' '.join(string.split()[0:-3]), x1y1, x2y2]


def create_square_grid(size):
    return [[0 for _ in range(size)] for _ in range(size)]


def right_command(command, value):
    if command == 'turn on':
        return value + 1
    if command == 'turn off':
        if value > 0:
            return value - 1
        else:
            return value
    return value + 2


def running_command(strings, grid):
    for string in strings:
        command = string[0]
        xy1 = string[1]
        xy2 = string[2]
        x1 = int(xy1[0])
        x2 = int(xy2[0])
        y1 = int(xy1[1])
        y2 = int(xy2[1])
        for x in range(x1, x2 + 1):
            for y in range(y1, y2 + 1):
                grid[x][y] = right_command(command, grid[x][y])
    return grid


def custom_sum(first, second):
    return first + second


def counting(grid):
    count = 0
    for x in grid:
        count += reduce(custom_sum, x)
    return count


def count_lights_after_commands(commands_str, grid_size):
    list_of_commands = list(map(splitting, commands_str.split('\n')))
    santa_grid = create_square_grid(grid_size)
    result_grid = running_command(list_of_commands, santa_grid)
    return counting(result_grid)


def main():
    puzzle_input = open('2015-6-input.txt').read().strip()
    result_count = count_lights_after_commands(puzzle_input, 1000)
    print(result_count)

result = main()
print(result)