from dataclasses import dataclass

@dataclass
class Command:
    command_type: str
    x1: int
    x2: int
    y1: int
    y2: int

    def execute(self, value):
        if self.command_type == 'turn on':
            return True
        if self.command_type == 'turn off':
            return False
        return not value


class DifferentArgs:
    def __init__(self, a, b=None):
        self.a = a
        self.b = b

DifferentArgs(2)
DifferentArgs(3, 2)


def splitting(string) -> Command:
    x1, y1 = (string.split()[-3]).split(',')
    x2, y2 = (string.split()[-1]).split(',')

    return Command(
        command_type=' '.join(string.split()[0:-3]),
        x1=int(x1),
        y1=int(y1),
        x2=int(x2),
        y2=int(y2)
    )


def create_square_grid(size):
    return [[False for _ in range(size)] for _ in range(size)]


def running_command(strings, grid):
    for command in strings:
        for x in range(command.x1, command.x2 + 1):
            for y in range(command.y1, command.y2 + 1):
                # grid[x][y] = right_command(command.command_type, grid[x][y])
                grid[x][y] = command.execute(grid[x][y])
    return grid


def counting(grid):
    count = 0
    for x in grid:
        count += x.count(True)
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


if __name__ == '__main__':
    main()
