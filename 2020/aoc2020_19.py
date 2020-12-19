import re

puzzle_input = open('aoc2020_19_input.txt').read().strip()


def first_puzzle(text):
    rules, messages = make_input(text)
    result = matches(0, rules)
    count = 0
    for message in messages:
        if message in result:
            count += 1
    print(count)


def second_puzzle(text):
    rules, messages = make_input(text)
    matches_42 = matches(42, rules)
    matches_31 = matches(31, rules)
    length = len(matches_42[0])
    messages_list = []
    for message in messages:
        if len(message) >= length * 3 and len(message)%length == 0:
            messages_list.append(message)
    valid = [x for x in messages_list if valid_message(x, matches_42, matches_31, length)]
    print(len(valid))


def make_input(text):
    temp_list = text.split('\n\n')
    rules_dict = {}
    for line in temp_list[0].split('\n'):
        result = re.search(r'([0-9]+): (.+)$', line)
        key = int(result.group(1))
        numbers = re.search(r'^(.+) \| (.+)$', result.group(2))
        if result.group(2) == '"a"' or result.group(2) == '"b"':
            rules_dict[key] = [result.group(2)[1]]
        elif numbers is not None:
            rules_dict[key] = [[int(x) for x in numbers.group(1).split(' ')], [int(x) for x in numbers.group(2).split(' ')]]
        else:
            rules_dict[key] = [[int(x) for x in result.group(2).split(' ')]]
    messages = temp_list[1].split('\n')
    return rules_dict, messages


def matches(rule, dict):
    matches_list = []
    if dict[rule][0] == 'a' or dict[rule][0] == 'b':
        return dict[rule]
    if len(dict[rule][0]) == 1:
        matches_list += matches(dict[rule][0][0], dict)
    else:
        for elem1 in matches(dict[rule][0][0], dict):
            for elem2 in matches(dict[rule][0][1], dict):
                matches_list.append(elem1+elem2)
    if len(dict[rule]) == 2:
        if len(dict[rule][1]) == 1:
            matches_list += matches(dict[rule][1][0], dict)
        else:
            for elem1 in matches(dict[rule][1][0], dict):
                for elem2 in matches(dict[rule][1][1], dict):
                    matches_list.append(elem1 + elem2)
    return matches_list


def valid_message(message, matches_42, matches_31, length):
    first_part = message[:length]
    second_part = message[length:length*2]
    last_part = message[-length:]
    if not first_part in matches_42 or not second_part in matches_42 or not last_part in matches_31:
        return False
    message = message[length*2:-length]
    part42 = 2
    part31 = 1
    can_be_42 = True
    while len(message) != 0:
        part = message[:length]
        message = message[length:]
        if can_be_42 and part in matches_42:
            part42 += 1
            continue
        elif part in matches_31:
            part31 += 1
            can_be_42 = False
            continue
        else:
            return False
    if part42 <= part31:
        return False
    return True


first_puzzle(puzzle_input)
second_puzzle(puzzle_input)