import re

commands_input = open('2015-7-input.txt').read().strip().split('\n')
commands_dict = {}

for command in commands_input:
    groups = re.search(r"(.*)\s+->\s+([a-z0-9]+)", command)
    commands_dict[groups.group(2)] = groups.group(1)

result = {}


def find_value(key, dictionary):
    global result
    def make_value(string):
        global result
        if string.isdigit():
            return int(string)
        elif string in result:
            return result[string]
        else:
            return find_value(string, dictionary)
    if key in result:
        return result[key]
    elif "OR" in dictionary[key]:
        groups = re.search(r"([a-z0-9]+)\s+OR\s+([a-z0-9]+)", dictionary[key])
        result[key] = make_value(groups.group(1)) | make_value(groups.group(2))
        return result[key]
    elif "AND" in dictionary[key]:
        groups = re.search(r"([a-z0-9]+)\s+AND\s+([a-z0-9]+)", dictionary[key])
        result[key] = make_value(groups.group(1)) & make_value(groups.group(2))
        return result[key]
    elif "RSHIFT" in dictionary[key]:
        groups = re.search(r"([a-z0-9]+)\s+RSHIFT\s+([a-z0-9]+)", dictionary[key])
        result[key] = make_value(groups.group(1)) >> make_value(groups.group(2))
        return result[key]
    elif "LSHIFT" in dictionary[key]:
        groups = re.search(r"([a-z0-9]+)\s+LSHIFT\s+([a-z0-9]+)", dictionary[key])
        result[key] = make_value(groups.group(1)) << make_value(groups.group(2))
        return result[key]
    elif "NOT" in dictionary[key]:
        groups = re.search(r"NOT\s+([a-z0-9]+)", dictionary[key])
        result[key] = 2**16 - 1 - make_value(groups.group(1))
        return result[key]
    else:
        result[key] = make_value(dictionary[key])
        return result[key]


print(find_value('a', commands_dict))
commands_dict['b'] = str(find_value('a', commands_dict))
result.clear()
print(find_value('a', commands_dict))