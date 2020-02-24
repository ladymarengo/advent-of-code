import re

commands_input = open('2015-7-input.txt').read().strip()
commands_list = commands_input.split('\n')

def real_action(input_list):
    result = {}
    commands = list(input_list)
    def make_value(string):
        nonlocal result
        if string.isdigit():
            return int(string)
        else:
            return result[string]
    def check_if_valid(string):
        nonlocal result
        if string in result or string.isdigit():
            return True
    while len(commands) > 0:
        for command in list(commands):
            if "AND" in command:
                groups = re.search(r"([a-z0-9]+)\s+AND\s+([a-z0-9]+)\s+->\s+([a-z0-9]+)", command)
                if check_if_valid(groups.group(1))  and check_if_valid(groups.group(2)):
                    first_value = make_value(groups.group(1))
                    second_value = make_value(groups.group(2))
                    result[groups.group(3)] = first_value & second_value
                    commands.remove(command)
                else:
                    continue
            elif "OR" in command:
                groups = re.search(r"([a-z0-9]+)\s+OR\s+([a-z0-9]+)\s+->\s+([a-z0-9]+)", command)
                if check_if_valid(groups.group(1))  and check_if_valid(groups.group(2)):
                    first_value = make_value(groups.group(1))
                    second_value = make_value(groups.group(2))
                    result[groups.group(3)] = first_value | second_value
                    commands.remove(command)
                else:
                    continue
            elif "RSHIFT" in command:
                groups = re.search(r"([a-z0-9]+)\s+RSHIFT\s+([0-9]+)\s+->\s+([a-z0-9]+)", command)
                if check_if_valid(groups.group(1)):
                    first_value = make_value(groups.group(1))
                    second_value = make_value(groups.group(2))
                    result[groups.group(3)] = first_value >> second_value
                    commands.remove(command)
                else:
                    continue
            elif "LSHIFT" in command:
                groups = re.search(r"([a-z0-9]+)\s+LSHIFT\s+([0-9]+)\s+->\s+([a-z0-9]+)", command)
                if check_if_valid(groups.group(1)):
                    first_value = make_value(groups.group(1))
                    second_value = make_value(groups.group(2))
                    result[groups.group(3)] = first_value << second_value
                    commands.remove(command)
                else:
                    continue
            elif "NOT" in command:
                groups = re.search(r"NOT\s+([a-z0-9]+)\s+->\s+([a-z0-9]+)", command)
                if check_if_valid(groups.group(1)):
                    first_value = make_value(groups.group(1))
                    result[groups.group(2)] = 2**16 - 1 - first_value
                    commands.remove(command)
                else:
                    continue
            else:
                groups = re.search(r"([a-z0-9]+)\s+->\s+([a-z0-9]+)", command)
                if check_if_valid(groups.group(1)):
                    first_value = make_value(groups.group(1))
                    result[groups.group(2)] = first_value
                    commands.remove(command)
    return result

result_list = real_action(commands_list)
print(result_list['a'])