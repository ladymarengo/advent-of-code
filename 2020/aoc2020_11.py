puzzle_input = open('aoc2020_11_input.txt').read().strip()

def main(text, task):
    seats = text.split('\n')
    new_seats = []

    def check_occupied(task, seats, row, seat):
        occupied = 0
        directions = ['up', 'upright', 'right', 'downright', 'down', 'downleft', 'left', 'upleft']
        if task == 'first':
            for i in range(row-1, row+2):
                for j in range(seat - 1, seat + 2):
                    if i >= 0 and i < len(seats) and j >= 0 and j < len(seats[0]) and seats[i][j] == '#' and not (i == row and j == seat):
                        occupied += 1
        elif task == 'second':
            for direction in directions:
                if check_direction(direction, seats, row, seat):
                    occupied += 1
        return occupied

    def check_direction(direction, seats, row, seat):
        i = row
        j = seat
        while i >= 0 and i < len(seats) and j >= 0 and j < len(seats[0]):
            if direction == 'up':
                i = i - 1
            elif direction == 'upright':
                i = i - 1
                j = j + 1
            elif direction == 'right':
                j = j + 1
            elif direction == 'downright':
                i = i + 1
                j = j + 1
            elif direction == 'down':
                i = i + 1
            elif direction == 'downleft':
                i = i + 1
                j = j - 1
            elif direction == 'left':
                j = j - 1
            elif direction == 'upleft':
                i = i - 1
                j = j - 1
            if i >= 0 and i < len(seats) and j >= 0 and j < len(seats[0]) and not (i == row and j == seat):
                if seats[i][j] == '#':
                    return True
                elif seats[i][j] == 'L':
                    return False
        return False

    def change_seats(seats, task):
        if task == 'first':
            amount = 4
        else:
            amount = 5
        nonlocal new_seats
        new_seats = []
        for i, line in enumerate(seats):
            new_line = []
            for j, seat in enumerate(line):
                if seat == 'L' and check_occupied(task, seats, i, j) == 0:
                    new_line.append('#')
                    change = True
                elif seat == '#' and check_occupied(task, seats, i, j) >= amount:
                    new_line.append('L')
                    change = True
                else:
                    new_line.append(seat)
            new_seats.append(''.join(new_line))
        if seats != new_seats:
            change_seats(new_seats, task)

    change_seats(seats, task)
    occupied_seats = len([seat for line in new_seats for seat in line if seat == '#'])
    print(occupied_seats)


test_input = '''L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL'''

main(test_input, 'first')
main(test_input, 'second')
main(puzzle_input, 'first')
main(puzzle_input, 'second')