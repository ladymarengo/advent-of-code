import re
from copy import deepcopy

puzzle_input = open('aoc2020_16_input.txt').read().strip()


def main(text, length):

    def split_input(text):
        result = re.search(r'([\s\S]+)\s+your ticket:\s([0-9,]+)\s+nearby tickets:\s([0-9,\s]+)', text)
        return result.group(1).strip(), result.group(2).strip().split(','), result.group(3).strip().split('\n')

    def make_fields(text):
        fields_dict = {}
        for line in text.split('\n'):
            result = re.search(r'(.+): (\d+)-(\d+) or (\d+)-(\d+)', line)
            fields_dict[Field(result.group(1), int(result.group(2)), int(result.group(3)),
                              int(result.group(4)), int(result.group(5)))] = None
        return fields_dict

    def check_tickets(ticket, fields):
        for value in ticket.split(','):
            invalid = True
            for field in fields:
                if ((int(value) >= field.min1 and int(value) <= field.max1) or (
                        int(value) >= field.min2 and int(value) <= field.max2)):
                    invalid = False
                    break
            if invalid:
                return False
        return True

    rules, my_ticket, tickets = split_input(text)
    fields = make_fields(rules)
    valid_tickets = [ticket for ticket in tickets if check_tickets(ticket, fields)]
    result = find_solution(fields, valid_tickets, length)
    multiply = 1
    for x in result:
        if 'departure' in x.name:
            multiply = multiply * int(my_ticket[result[x]])
    print(multiply)


class Field():
    def __init__(self, name, min1, max1, min2, max2):
        self.name = name
        self.min1 = min1
        self.max1 = max1
        self.min2 = min2
        self.max2 = max2

    def __eq__(self, other):
        return self.name == other.name

    def __hash__(self):
        return hash(self.name)

    def __repr__(self):
        return f"Field({self.name})"


def find_solution(fields, tickets, length):
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
        tickets_dict[ticket] = node_consistent(tickets_dict[ticket])

    result = backtrack(tickets_dict, dict(), fields, length)
    return result


def node_consistent(fields):
    new_fields = {}
    for field in fields:
        new_list = []
        for value in list(fields[field]):
            if ((fields[field][value] >= field.min1 and fields[field][value] <= field.max1) or (
                    fields[field][value] >= field.min2 and fields[field][value] <= field.max2)):
                new_list.append(value)
        new_fields[field] = new_list
    return new_fields


def order_domain_values(length, assignment):
    values = list(range(int(length)))
    if len(assignment) != 0:
        for var in assignment:
            values.remove(assignment[var])
    return values


def select_unassigned_variable(assignment, fields, tickets):
    remaining_values = {}
    for var in fields:
        if not var in assignment:
            sum = 0
            for ticket in tickets:
                sum += len(tickets[ticket][var])
            remaining_values[sum] = var
    return remaining_values[min(remaining_values)]


def assignment_complete(assignment, fields):
    for var in fields:
        if not var in assignment:
            return False
    return True


def revise(x, y, ticket, tickets):
    new_x_list = [value for value in tickets[ticket][x] if set(tickets[ticket][y]) - {value}]
    if len(new_x_list) != len(tickets[ticket][x]):
        tickets[ticket][x] = new_x_list
        return True
    return False


def ac3(arcs, ticket, tickets):
    while len(arcs) != 0:
        (var, reference) = arcs.pop(0)
        if revise(var, reference, ticket, tickets):
            if len(tickets[ticket][var]) == 0:
                return False
            for neighbour in tickets[ticket]:
                if neighbour != reference and neighbour != var:
                    arcs.append((neighbour, var))
    return True


def consistent(ticket, tickets, var):
    arcs = []
    for other_var in tickets[ticket]:
        if var != other_var:
            arcs.append((other_var, var))
    if ac3(arcs, ticket, tickets):
        return True
    return False


def tickets_consistency(assignment, tickets, fields, var):
    for ticket in tickets:
        if not assignment[var] in tickets[ticket][var]:
            return False
        tickets[ticket][var] = [assignment[var]]
        if not consistent(ticket, tickets, var):
            return False
    return True


def backtrack(tickets, assignment, fields, length):
    if assignment_complete(assignment, fields):
        return assignment
    new_var = select_unassigned_variable(assignment, fields, tickets)
    for value in order_domain_values(length, assignment):
        previous_domain = deepcopy(tickets)
        assignment[new_var] = value
        if tickets_consistency(assignment, tickets, fields, new_var):
            result = backtrack(tickets, assignment, fields, length)
            if result != None:
                return result
        assignment.pop(new_var)
        tickets = previous_domain
    return None


main(puzzle_input, 20)