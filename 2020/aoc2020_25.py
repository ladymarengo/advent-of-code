puzzle_input = '18356117 5909654'


def main(text):
    card_key = text.split(' ')[0]
    door_key = text.split(' ')[1]
    card_loop_size = find_loop_size(card_key, 7)
    door_loop_size = find_loop_size(door_key, 7)
    card_encryption_key = find_encryption_key(int(door_key), card_loop_size)
    door_encryption_key = find_encryption_key(int(card_key), door_loop_size)
    print(card_encryption_key, door_encryption_key)


def find_loop_size(key, subject_number):
    value = 1
    loop_size = 0
    while value != int(key):
        value = value * subject_number
        value = value % 20201227
        loop_size += 1
    return loop_size


def find_encryption_key(subject_number, loop_size):
    value = 1
    for i in range(loop_size):
        value = value * subject_number
        value = value % 20201227
    return value


main(puzzle_input)