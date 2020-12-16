import re
import random

puzzle_input = open('aoc2020_16_input.txt').read().strip()

def main(text):
    rules, my_ticket, tickets = split_input(text)
    rules, fields = make_rules(rules)
    valid_tickets = [ticket for ticket in tickets if check_tickets(ticket, rules)]
    find_fields_information(fields, valid_tickets)

def split_input(text):
    result = re.search(r'([\s\S]+)\s+your ticket:\s([0-9,]+)\s+nearby tickets:\s([0-9,\s]+)', text)
    return result.group(1).strip(), result.group(2).strip().split(','), result.group(3).strip().split('\n')

def make_rules(text):
    rules_dict = {}
    fields_dict = {}
    for line in text.split('\n'):
        result = re.search(r'(.+): (\d+)-(\d+) or (\d+)-(\d+)', line)
        rules_dict[result.group(1)] = [int(result.group(2)), int(result.group(3)),
                                       int(result.group(4)), int(result.group(5))]
        fields_dict[Field(result.group(1), int(result.group(2)), int(result.group(3)),
                                       int(result.group(4)), int(result.group(5)))] = None
    return rules_dict, fields_dict

def check_tickets(ticket, rules):
    for value in ticket.split(','):
        if check_invalid(int(value), rules):
            return False
    return True

def check_invalid(value, rules):
    for rule in rules:
        if (value >= rules[rule][0] and value <= rules[rule][1]) or (value >= rules[rule][2] and value <= rules[rule][3]):
            return False
    return True

class Field():
    def __init__(self, name, min1, max1, min2, max2):
        self.name = name
        self.min1 = min1
        self.max1 = max1
        self.min2 = min2
        self.max2 = max2

def find_fields_information(fields, tickets):
    ticket = [int(x) for x in tickets[0].split(',')]
    # print(ticket)
    for field in fields:
        fields[field] = set([(e, x) for e, x in enumerate(ticket)])
    # print(fields)
    fields = node_consistent(fields)
    print(fields)
    result = backtrack(dict(), fields, tickets)
    # print(result)
    for x in result:
        print(x.name, result[x])

def node_consistent(fields):
    for field in fields:
        for value in list(fields[field]):
            if not ((value[1] >= field.min1 and value[1] <= field.max1) or (
                    value[1] >= field.min2 and value[1] <= field.max2)):
                fields[field].remove(value)
    return fields

def invalid(index, field, ticket):
    value = int(ticket[index])
    if (value >= field.min1 and value <= field.max1) or (
                value >= field.min2 and value <= field.max2):
        return False
    return True

def consistent(assignment, value):
    count = 0
    for field in assignment:
        if assignment[field] == value:
            count += 1
            if count > 1:
                return False
    return True

def consistent_with_other_tickets(value, tickets, field):
    for ticket in tickets:
        if invalid(value[0], field, ticket.split(',')):
            return False
    return True

def backtrack(assignment, fields, tickets):
    # print(len(assignment))
    if len(assignment) == len(fields):
        return assignment
    var = random.choice([x for x in fields if not x in assignment])
    # print(var.name)
    for value in fields[var]:
        # print(value)
        if consistent_with_other_tickets(value, tickets, var):
            assignment[var] = value
            if consistent(assignment, value):
                # print('continue')
                # print(assignment)
                result = backtrack(assignment, fields, tickets)
                if result != None:
                    # print('going back')
                    return result
            assignment.pop(var)
    return None


test_input = '''class: 1-3 or 5-7
row: 6-11 or 33-44
seat: 13-40 or 45-50

your ticket:
7,1,14

nearby tickets:
7,3,47
40,4,50
55,2,20
38,6,12'''

test_input2 = '''class: 0-1 or 4-19
row: 0-5 or 8-19
seat: 0-13 or 16-19

your ticket:
11,12,13

nearby tickets:
3,9,18
15,1,5
5,14,9'''

main(test_input2)
# main(puzzle_input)