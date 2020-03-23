import re
task_input = open('2015_19_input.txt').read().strip()


def make_input(text):
    replacements_strings = []
    replacements_keys = []
    molecule = ''

    for line in text.strip().split('\n'):
        if '=>' in line:
            replacements_strings.append(line)
        elif len(line) == 0:
            pass
        else:
            molecule = line

    def process(string):
        data = re.search(r"(?P<original>[A-Za-z]+)\s=>\s(?P<replaced>[A-Za-z]+)", string)
        if data.group('original') not in replacements_keys:
            replacements_keys.append(data.group('original'))
        return data.group('original'), data.group('replaced')

    replacements = [process(replacement) for replacement in replacements_strings]
    return replacements, replacements_keys, molecule


def make_replacements(replacements, keys, molecule):
    list_of_molecules = []

    def making_new_molecule(first_part, second_part, new_char):
        nonlocal list_of_molecules
        new_molecule = first_part + new_char + second_part
        list_of_molecules.append(new_molecule)

    for i, char in enumerate(molecule):
        if char in keys:
            for replacement in replacements:
                if replacement[0] == char:
                    making_new_molecule(molecule[:i], molecule[i+1:], replacement[1])
        elif i < (len(molecule) - 1) and ''.join([char, molecule[i + 1]]) in keys:
            for replacement in replacements:
                if replacement[0] == ''.join([char, molecule[i + 1]]):
                    making_new_molecule(molecule[:i], molecule[i+2:], replacement[1])

    return len(set(list_of_molecules))


def main(text_input):
    replacements, keys, molecule = make_input(text_input)
    return make_replacements(replacements, keys, molecule)


if __name__ == '__main__':
    result = main(task_input)
    print(result)