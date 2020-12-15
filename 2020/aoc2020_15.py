puzzle_input = '1,20,11,6,12,0'


def main(text, final):
    first_numbers = text.split(',')
    numbers_dict = {}
    numbers_list = []

    for i, number in enumerate(first_numbers):
        numbers_dict[int(number)] = i
        numbers_list.append(int(number))

    for i in range(len(first_numbers), final):
        look_at_number = numbers_list[i-1]
        if not look_at_number in numbers_dict or numbers_dict[look_at_number] == i-1:
            new_number = 0
        else:
            new_number = (i-1) - numbers_dict[look_at_number]
        numbers_dict[look_at_number] = i-1
        numbers_list.append(new_number)

    print(numbers_list[final-1])


main(puzzle_input, 2020)
main(puzzle_input, 30000000)