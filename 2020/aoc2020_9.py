puzzle_input = open('aoc2020_9_input.txt').read().strip()


def main(text, amount):
    numbers = text.split('\n')
    weakness_number = []

    def check(numbers, check_index, amount):
        for j in range(check_index-amount, check_index):
            for k in range(check_index-amount, check_index):
                if j != k and int(numbers[j]) + int(numbers[k]) == int(numbers[check_index]):
                    return True
        return False

    def check_sum(number, numbers, index):
        sum = int(numbers[index])
        sum_list = [int(numbers[index])]
        while sum < int(number):
            index += 1
            sum_list.append(int(numbers[index]))
            sum += int(numbers[index])
            if sum == int(number):
                return sum_list
        return None

    for i in range(amount, len(numbers)):
        if not check(numbers, i, amount):
            print('answer for first puzzle is ', numbers[i])
            weakness_number.extend([numbers[i], i])
            break

    for i in range(0, weakness_number[1]):
        result = check_sum(weakness_number[0], numbers, i)
        if not result == None:
            print('answer for second puzzle is ', min(result) + max(result))
            break


test_input = '''35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576'''

main(test_input, 5)
main(puzzle_input, 25)