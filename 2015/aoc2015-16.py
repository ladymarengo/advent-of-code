from dataclasses import dataclass, fields
import re

test_input = '''Sue 7: trees: 2, samoyeds: 7, goldfish: 10
Sue 8: cars: 8, perfumes: 6, goldfish: 1
Sue 9: cats: 4, pomeranians: 0, trees: 0'''

real_input = open('2015-16-input.txt').read()

true_aunt_dict = {'children': 3, 'cats': 7, 'samoyeds': 2, 'pomeranians': 3, 'akitas': 0,
                  'vizslas': 0, 'goldfish': 5, 'trees': 3, 'cars': 2, 'perfumes': 1}


@dataclass
class Aunt:
    number: int = None
    children: int = None
    cats: int = None
    samoyeds: int = None
    pomeranians: int = None
    akitas: int = None
    vizslas: int = None
    goldfish: int = None
    trees: int = None
    cars: int = None
    perfumes: int = None

    @classmethod
    def from_string(cls, string):
        information = re.search(r"Sue\s(?P<number>[0-9]+):\s(?P<first_atr>[a-z]+):\s(?P<first_value>[0-9]+)\,"
                                r"\s(?P<second_atr>[a-z]+):\s(?P<second_value>[0-9]+)\,\s(?P<third_atr>[a-z]+)"
                                r":\s(?P<third_value>[0-9]+)", string)
        inf_dict = {}
        inf_dict['number'] = int(information.group('number'))
        inf_dict[information.group('first_atr')] = int(information.group('first_value'))
        inf_dict[information.group('second_atr')] = int(information.group('second_value'))
        inf_dict[information.group('third_atr')] = int(information.group('third_value'))
        return cls(**inf_dict)


def making_true_aunt(dict):
    return Aunt(**dict)


def making_list_of_aunts(raw_input):
    list_of_aunts = [Aunt.from_string(x) for x in raw_input.strip().split('\n')]
    return list_of_aunts


def checking_aunts(true_aunt, aunts):
    final_aunts = list(aunts)
    attributes = [f.name for f in fields(true_aunt)]
    attributes.remove('number')
    for aunt in aunts:
        for attribute in attributes:
            if aunt in final_aunts and getattr(aunt, attribute) != None:
                if attribute == 'cats' or attribute == 'trees':
                    if getattr(aunt, attribute) <= getattr(true_aunt, attribute):
                        final_aunts.remove(aunt)
                elif attribute == 'pomeranians' or attribute == 'goldfish':
                    if getattr(aunt, attribute) >= getattr(true_aunt, attribute):
                        final_aunts.remove(aunt)
                elif getattr(aunt, attribute) != getattr(true_aunt, attribute):
                    final_aunts.remove(aunt)
                    break
    return final_aunts


def main(raw_input, true_dict):
    true_aunt = making_true_aunt(true_dict)
    aunts = making_list_of_aunts(raw_input)
    result = checking_aunts(true_aunt, aunts)
    return result

a = main(real_input, true_aunt_dict)
print(a)