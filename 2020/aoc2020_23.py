puzzle_input = '653427918'


def first_puzzle(text, rounds):

    numbers = [int(x) for x in text]

    def solve(numbers, rounds):
        current_cup = numbers[0]

        def get_index(current_index, numbers):
            if current_index == len(numbers) - 1:
                return 0
            else:
                return current_index + 1

        for round in range(rounds):
            three_cups = []
            for i in range(3):
                three_cups.append(numbers.pop(get_index(numbers.index(current_cup), numbers)))
            destination_cup = current_cup - 1
            while not destination_cup in numbers:
                destination_cup -= 1
                if destination_cup < min(numbers):
                    destination_cup = max(numbers)
            destination_index = numbers.index(destination_cup)
            for cup in reversed(three_cups):
                numbers.insert(destination_index + 1, cup)
            current_cup = numbers[get_index(numbers.index(current_cup), numbers)]

        return numbers

    numbers = solve(numbers, rounds)
    while numbers[0] != 1:
        number = numbers.pop(0)
        numbers.append(number)
    numbers.pop(0)
    answer = ''.join([str(x) for x in numbers])
    print(answer)


class Cup:
    def __init__(self, number, next_cup=None):
        self.number = number
        self.next = next_cup

    def __repr__(self):
        return f'Cup({self.number})'


def parse_cuptable(text):
    cup_dict = {}
    previous_cup = None
    for symbol in text:
        cup = Cup(int(symbol))
        cup_dict[int(symbol)] = cup
        if previous_cup is not None:
            previous_cup.next = cup
        previous_cup = cup
    cup_dict[int(text[-1])].next = cup_dict[int(text[0])]
    return cup_dict


def make_million_cups(first_cup, last_cup, cups):
    previous_cup = last_cup
    for i in range(max(cups) + 1, 1000000 + 1):
        cups[i] = Cup(i)
        cups[previous_cup].next = cups[i]
        previous_cup = cups[i].number
    cups[previous_cup].next = cups[first_cup]
    return cups


def second_puzzle(text, rounds):
    cups = parse_cuptable(text)
    million_cups = make_million_cups(int(text[0]), int(text[-1]), cups)
    after = solve(int(text[0]), million_cups, rounds)
    print(after[1].next.number * after[1].next.next.number)


def solve(first_cup, cups, rounds):
    current_cup = cups[first_cup]
    for i in range(rounds):
        three_cups = []
        cup = current_cup
        for i in range(3):
            cup = cup.next
            three_cups.append(cup.number)
        current_cup.next = cup.next

        destination_cup = current_cup.number-1
        while destination_cup in three_cups or destination_cup == current_cup.number or destination_cup == 0:
            destination_cup -= 1
            if destination_cup < 1:
                destination_cup = max(cups)

        temp_next = cups[destination_cup].next
        cups[destination_cup].next = cups[three_cups[0]]
        cups[three_cups[-1]].next = temp_next

        current_cup = current_cup.next

    return cups


first_puzzle(puzzle_input, 100)
second_puzzle(puzzle_input, 10000000)

