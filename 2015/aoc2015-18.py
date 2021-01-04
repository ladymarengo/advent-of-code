from textwrap import dedent
from time import time
from typing import Dict, Tuple

task_input = open('2015-18-input.txt').read().strip()


class Grid:
    _grid: Dict[Tuple[int, int], bool]

    def __init__(self, grid, grid_side_size):
        self._grid = grid
        self._side_size = grid_side_size

    @classmethod
    def from_string(cls, data):
        grid = {}
        data_lines = data.strip().split('\n')
        grid_side_size = len(data_lines)
        for y, line in enumerate(data_lines):
            for x, char in enumerate(list(line)):
                if char == '#':
                    val = True
                elif char == '.':
                    val = False
                else:
                    raise ValueError(f"Unknown value {char} at {x}, {y}")
                grid[(x, y)] = val
        return cls(grid, grid_side_size)

    def _calculate_alive_neighbours(self, x, y):
        alive = 0
        for neighbour_x in range(x - 1, x + 2):
            for neighbour_y in range(y - 1, y + 2):
                if (neighbour_x, neighbour_y) != (x, y):
                    if self._grid.get((neighbour_x, neighbour_y), None):
                        alive += 1
        return alive

    def step(self):
        new_grid = {}
        for y in range(self._side_size):
            for x in range(self._side_size):
                alive_neighbours = self._calculate_alive_neighbours(x, y)

                if self._grid[(x, y)]:
                    new_value = (2 <= alive_neighbours <= 3)
                else:
                    new_value = (alive_neighbours == 3)
                new_grid[(x, y)] = new_value
        self._grid = new_grid

    def to_string(self):
        result = []
        for y in range(self._side_size):
            for x in range(self._side_size):
                result.append('#' if self._grid[(x, y)] else '.')
            result.append('\n')
        return ''.join(result)

    def calculate_alive(self):
        return sum(1 if light else 0 for light in self._grid.values())


class StuckGrid(Grid):

    def __init__(self, data, size):
        super(StuckGrid, self).__init__(data, size)
        self._light_up_stuck()

    def _light_up_stuck(self):
        for x in (0, self._side_size - 1):
            for y in (0, self._side_size - 1):
                self._grid[(x, y)] = True

    def step(self):
        super(StuckGrid, self).step()
        self._light_up_stuck()


def test_neighbours():
    grid = Grid.from_string(dedent('''
        .#.#.#
        ...##.
        #....#
        ..#...
        #.#..#
        ####..
    '''))
    assert grid._calculate_alive_neighbours(0, 0) == 1
    assert grid._calculate_alive_neighbours(2, 4) == 4


def main():
    start = time()
    grid = Grid.from_string(task_input)

    for _ in range(100):
        start = time()
        grid.step()

    stuck_grid = StuckGrid.from_string(task_input)
    for _ in range(100):
        stuck_grid.step()


if __name__ == '__main__':
    main()


def test_provided():
    grid = Grid.from_string(dedent('''
        .#.#.#
        ...##.
        #....#
        ..#...
        #.#..#
        ####..
    '''))

    grid.step()

    assert grid.to_string().strip() == dedent('''
        ..##..
        ..##.#
        ...##.
        ......
        #.....
        #.##..
    ''').strip()

    grid.step()

    assert grid.to_string().strip() == dedent('''
        ..###.
        ......
        ..###.
        ......
        .#....
        .#....
    ''').strip()

    grid.step()
    grid.step()

    assert grid.calculate_alive() == 4

STUCK_GRID_STAGES = [
    '''
        #.##.#
        ####.#
        ...##.
        ......
        #...#.
        #.####
    ''',
    '''
        #..#.#
        #....#
        .#.##.
        ...##.
        .#..##
        ##.###
    ''',
    '''
        #...##
        ####.#
        ..##.#
        ......
        ##....
        ####.#
    ''',
    '''
        #.####
        #....#
        ...#..
        .##...
        #.....
        #.#..#    
    ''',
    '''
        ##.###
        .##..#
        .##...
        .##...
        #.#...
        ##...#
    '''
]


def test_new_grid():
    g = StuckGrid.from_string(dedent('''
        ##.#.#
        ...##.
        #....#
        ..#...
        #.#..#
        ####.#
    '''))

    for i, stage in enumerate(STUCK_GRID_STAGES):
        g.step()
        assert g.to_string().strip() == dedent(stage).strip(), f"stage {i} failed"

    assert g.calculate_alive() == 17
