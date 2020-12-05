puzzle_input = open('aoc2020_5_input.txt').read().strip()

test_input = '''FBFBBFFRLR
BFFFBBFRRR
FFFBBBFRRR
BBFFBBFRLL'''

def main(seats):
    id_list = []
    for seat in seats.split('\n'):
        row_seats_amount = 128
        row_last_seat = 128
        column_seats_amount = 8
        column_last_seat = 8
        for i in range(7):
            row_seats_amount, row_last_seat = check(row_seats_amount, row_last_seat, seat[i])
        row = int(row_last_seat) - 1
        for i in range(7, 10):
            column_seats_amount, column_last_seat = check(column_seats_amount, column_last_seat, seat[i])
        column = int(column_last_seat) - 1
        id = row * 8 + column
        # print(seat, row, column, id)
        id_list.append(id)
    missing = check_missing(id_list)
    print(missing)

def check(amount, last, action):
    value = amount/2
    if action == 'F' or action == 'L':
        return value, last - value
    elif action == 'B' or action == 'R':
        return value, last

def check_missing(id_list):
    missing_ids = []
    for i in range(9, 1024):
        if i not in id_list and i+1 in id_list and i-1 in id_list:
            missing_ids.append(i)
    return missing_ids

main(test_input)
main(puzzle_input)