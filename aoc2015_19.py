import re
task_input = open('2015_19_input.txt').read().strip()
test_input = '''e => H
e => O
H => HO
H => OH
O => HH

HOH'''


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


def make_one_replacement(replacements, keys, molecule):
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

    return list_of_molecules


def counting_steps(replacements, keys, final_molecule, list_of_molecules, counter = 0):
    counter += 1
    total_new_list = []
    for molecule in list_of_molecules:
        new_list = make_one_replacement(replacements, keys, molecule)
        total_new_list.extend(new_list)
    new_set = set(total_new_list)
    for molecule in new_set:
        if molecule == final_molecule:
            return counter
    return counting_steps(replacements, keys, final_molecule, new_set, counter)


def making_full_molecule(replacements, keys, final_molecule):
    molecule = 'e'
    global_counter = None

    def making_molecule(first_part, second_part, new_char):
        new_molecule = first_part + new_char + second_part
        return new_molecule

    def check_molecule(new_molecule, final_molecule):
        return new_molecule == final_molecule

    def one_step(replacements, keys, molecule, final_molecule, counter = 0):
        new_counter = counter + 1
        nonlocal global_counter

        for i, char in enumerate(molecule):
            if char in keys:
                for replacement in replacements:
                    if replacement[0] == char:
                        new_molecule = making_molecule(molecule[:i], molecule[i+1:], replacement[1])
                        if check_molecule(new_molecule, final_molecule):
                            if global_counter == None or new_counter < global_counter:
                                global_counter = new_counter
                        elif len(molecule) < len(final_molecule):
                            one_step(replacements, keys, new_molecule, final_molecule, new_counter)
            elif i < (len(molecule) - 1) and ''.join([char, molecule[i + 1]]) in keys:
                for replacement in replacements:
                    if replacement[0] == ''.join([char, molecule[i + 1]]):
                        new_molecule = making_molecule(molecule[:i], molecule[i+2:], replacement[1])
                        if check_molecule(new_molecule, final_molecule):
                            if global_counter == None or new_counter < global_counter:
                                global_counter = new_counter
                        elif len(molecule) < len(final_molecule):
                            one_step(replacements, keys, new_molecule, final_molecule, new_counter)

    one_step(replacements, keys, molecule, final_molecule)
    return global_counter


def main(text_input):
    replacements, keys, molecule = make_input(text_input)
    list_of_molecules = ['e']
    return counting_steps(replacements, keys, molecule, list_of_molecules)


if __name__ == '__main__':
    result = main(task_input)
    print(result)