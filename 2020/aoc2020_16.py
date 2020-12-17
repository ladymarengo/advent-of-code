import re
import random

puzzle_input = open('aoc2020_16_input.txt').read().strip()


def main(text):

    def split_input(text):
        result = re.search(r'([\s\S]+)\s+your ticket:\s([0-9,]+)\s+nearby tickets:\s([0-9,\s]+)', text)
        return result.group(1).strip(), result.group(2).strip().split(','), result.group(3).strip().split('\n')

    def make_rules(text):
        fields_dict = {}
        for line in text.split('\n'):
            result = re.search(r'(.+): (\d+)-(\d+) or (\d+)-(\d+)', line)
            fields_dict[Field(result.group(1), int(result.group(2)), int(result.group(3)),
                              int(result.group(4)), int(result.group(5)))] = None
        return fields_dict

    def check_tickets(ticket, fields):
        invalid = False
        for value in ticket.split(','):
            invalid = False
            for field in fields:
                if ((int(value) >= field.min1 and int(value) <= field.max1) or (
                        int(value) >= field.min2 and int(value) <= field.max2)):
                    invalid = True
        return invalid

    rules, my_ticket, tickets = split_input(text)
    fields = make_rules(rules)
    valid_tickets = [ticket for ticket in tickets if check_tickets(ticket, fields)]
    # find_fields_information(fields, valid_tickets)
    another_way(fields, valid_tickets)


class Field():
    def __init__(self, name, min1, max1, min2, max2):
        self.name = name
        self.min1 = min1
        self.max1 = max1
        self.min2 = min2
        self.max2 = max2

# def find_fields_information(fields, tickets):
#     ticket = [int(x) for x in tickets[0].split(',')]
#     # print(ticket)
#     for field in fields:
#         fields[field] = set([(e, x) for e, x in enumerate(ticket)])
#     # print(fields)
#     fields = node_consistent(fields)
#     # print(fields)
#     result = backtrack(dict(), fields, tickets)
#     print(result)
#     for x in result:
#         print(x.name, result[x])

def another_way(fields, tickets):
    tickets_dict = {}
    for i, ticket in enumerate(tickets):
        int_ticket = [int(x) for x in ticket.split(',')]
        fields_dict = {}
        for field in fields:
            ticket_dict = {}
            for e, x in enumerate(int_ticket):
                ticket_dict[e] = x
            fields_dict[field] = ticket_dict
        tickets_dict[i] = fields_dict
    for ticket in tickets_dict:
        tickets_dict[ticket] = another_node_consistent(tickets_dict[ticket])
    # print(tickets_dict)
    result = another_backtrack(dict(), fields, tickets_dict)
    for x in result:
        print(x.name, result[x])

def consistance_another_way(value, var, tickets):
    # print(tickets)
    for ticket in tickets:
        # print(value, tickets[ticket][var])
        if not value in tickets[ticket][var]:
            return False
    return True

def another_backtrack(assignment, fields, tickets):
    if len(assignment) == len(fields):
        return assignment
    var = [x for x in fields if not x in assignment][0]
    for value in range(len(fields)):
        if consistance_another_way(value, var, tickets):
            assignment[var] = value
            if consistent(assignment, value):
                print('continue')
                # print(assignment)
                result = another_backtrack(assignment, fields, tickets)
                if result != None:
                    print('going back')
                    return result
            assignment.pop(var)
    return None

def another_node_consistent(fields):
    for field in fields:
        for value in list(fields[field]):
            if not ((fields[field][value] >= field.min1 and fields[field][value] <= field.max1) or (
                    fields[field][value] >= field.min2 and fields[field][value] <= field.max2)):
                fields[field].pop(value)
    return fields

def consistent(assignment, value):
    count = 0
    for field in assignment:
        if assignment[field] == value:
            count += 1
            if count > 1:
                return False
    return True

# def node_consistent(fields):
#     for field in fields:
#         for value in list(fields[field]):
#             if not ((value[1] >= field.min1 and value[1] <= field.max1) or (
#                     value[1] >= field.min2 and value[1] <= field.max2)):
#                 fields[field].remove(value)
#     return fields

# def invalid(index, field, ticket):
#     value = int(ticket[index])
#     print(value, field.min1, field.max1, field.min2, field.max2)
#     if (value >= field.min1 and value <= field.max1) or (
#                 value >= field.min2 and value <= field.max2):
#         print('not invalid')
#         return False
#     print('invalid')
#     return True



# def consistent_with_other_tickets(value, tickets, field):
#     # for ticket in tickets:
#     #     if invalid(value[0], field, ticket.split(',')):
#     #         return False
#     return True

# def backtrack(assignment, fields, tickets):
#     print(len(assignment))
#     if len(assignment) == len(fields):
#         return assignment
#     var = random.choice([x for x in fields if not x in assignment])
#     print(var.name)
#     for value in fields[var]:
#         print(value)
#         if consistent_with_other_tickets(value, tickets, var):
#             assignment[var] = value
#             if consistent(assignment, value):
#                 print('continue')
#                 print(assignment)
#                 result = backtrack(assignment, fields, tickets)
#                 if result != None:
#                     print('going back')
#                     return result
#             assignment.pop(var)
#     return None


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

test_input3 = open('aoc2020_16_test_input.txt').read().strip()

main(test_input3)
# main(puzzle_input)