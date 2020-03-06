from dataclasses import dataclass
import re

test_input = '''Sue 7: trees: 2, samoyeds: 7, goldfish: 10
Sue 8: cars: 8, perfumes: 6, goldfish: 1
Sue 9: cats: 4, pomeranians: 0, trees: 0'''


@dataclass
class Aunt:
    number: int
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


def making_list_of_aunts(raw_input):
    list_of_aunts = [Aunt.from_string(x) for x in raw_input.strip().split('\n')]
    return list_of_aunts


a = making_list_of_aunts(test_input)
print(a)