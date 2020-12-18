import re

puzzle_input = open('aoc2020_18_input.txt').read().strip()


def main(expressions):
    first_solutions = [first_solution(expression) for expression in expressions]
    first_sum = 0
    for number in first_solutions:
        first_sum += number
    print('first puzzle answer is ', first_sum)

    second_solutions = [second_solution(expression) for expression in expressions]
    second_sum = 0
    for number in second_solutions:
        second_sum += number
    print('second puzzle answer is ', second_sum)


def first_solution(expression):
    number = None
    action = ''
    in_parentheses = False
    parentheses_amount = 0
    inner_expr = ''

    for symbol in expression:
        if symbol == ')':
            parentheses_amount -= 1
            if parentheses_amount == 0:
                in_parentheses = False
                if number is None:
                    number = first_solution(inner_expr)
                else:
                    if action == '+':
                        number += first_solution(inner_expr)
                    else:
                        number = number * first_solution(inner_expr)
                inner_expr = ''
                continue
        if in_parentheses:
            inner_expr += symbol
            if symbol == '(':
                parentheses_amount += 1
        elif symbol == ' ':
            continue
        elif symbol == '+' or symbol == '*':
            action = symbol
        elif symbol == '(':
            in_parentheses = True
            parentheses_amount += 1
        else:
            if number is None:
                number = int(symbol)
            else:
                if action == '+':
                    number += int(symbol)
                else:
                    number = number * int(symbol)
    return number


def second_solution(expression):
    exp_list = []
    in_parentheses = False
    parentheses_amount = 0
    inner_expr = ''

    for symbol in expression:
        if symbol == ')':
            parentheses_amount -= 1
            if parentheses_amount == 0:
                in_parentheses = False
                exp_list.append(second_solution(inner_expr))
                inner_expr = ''
                continue
        if in_parentheses:
            inner_expr += symbol
            if symbol == '(':
                parentheses_amount += 1
        elif symbol == ' ':
            continue
        elif symbol == '(':
            in_parentheses = True
            parentheses_amount += 1
        else:
            exp_list.append(symbol)

    new_list = []
    while len(exp_list) != 0:
        elem = exp_list.pop(0)
        if len(exp_list) > 1 and exp_list[0] == '+':
            sum = int(elem) + int(exp_list[1])
            exp_list.pop(0)
            exp_list.pop(0)
            exp_list.insert(0, sum)
        elif elem != '*':
            new_list.append(elem)

    result = 1
    for number in new_list:
        result = result * int(number)
    return result

main(puzzle_input.strip().split('\n'))