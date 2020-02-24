import json

json_string = open('2015-12-input.txt').read()
json_structure = json.loads(json_string)

counter = 0
def counting(structure):
    global counter
    if isinstance(structure, int):
        counter += structure
    elif isinstance(structure, str):
        return counter
    elif isinstance(structure, dict):
        for key in structure:
            counting(structure[key])
    elif isinstance(structure, list):
        for key in structure:
            counting(key)
    return counter


counter = 0
def counting_without_red(structure):
    global counter
    def checking_for_red(dict):
        for key in dict:
            if key == 'red' or dict[key] == 'red':
                return False
        return True
    if isinstance(structure, int):
        counter += structure
    elif isinstance(structure, str):
        return counter
    elif isinstance(structure, dict):
        if checking_for_red(structure):
            for key in structure:
                counting_without_red(structure[key])
    elif isinstance(structure, list):
        for key in structure:
            counting_without_red(key)
    return counter


res = counting_without_red(json_structure)
print('sum = ', res)