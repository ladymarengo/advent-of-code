import re


def main(text):
    required_fields = ['byr', 'iyr', 'eyr', 'hgt', 'hcl', 'ecl', 'pid']
    info_list = text.split('\n\n')
    valid_passports = 0
    for line in info_list:
        fields = line.split()
        fields_count = 0
        for field in fields:
            res = re.search(r"([0-9a-z#]+):([0-9a-z#]+)", field)
            if res.group(1) in required_fields and valid(res.group(1), res.group(2)):
                fields_count += 1
        if fields_count == 7:
            valid_passports += 1
    print(valid_passports)

def valid(field, value):
    if field == 'byr' and int(value) >= 1920 and int(value) <= 2002:
        return True
    elif field == 'iyr' and int(value) >= 2010 and int(value) <= 2020:
        return True
    elif field == 'eyr' and int(value) >= 2020 and int(value) <= 2030:
        return True
    elif field == 'hgt':
        res = re.search(r"([0-9]+)([a-z]+)", value)
        if res is not None:
            height = int(res.group(1))
            unit = res.group(2)
            if (unit == 'cm' and height >= 150 and height <= 193) or (unit == 'in' and height >= 59 and height <= 76):
                return True
    elif field == 'hcl':
        res = re.search(r"#([0-9a-z]+)", value)
        if res is not None and len(res.group(1)) == 6:
            return True
    elif field == 'ecl':
        allowed_colors = ['amb', 'blu', 'brn', 'gry', 'grn', 'hzl', 'oth']
        if value in allowed_colors:
            return True
    elif field == 'pid':
        res = re.search(r"^([0-9]+)$", value)
        if res is not None and len(res.group(1)) == 9:
            return True
    elif field == 'cid':
        return True


puzzle_input = open('aoc2020_4_input.txt').read().strip()

test_input = '''ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in'''

valid_passports = '''pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719'''

invalid_passports = '''eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946

hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007'''

main(invalid_passports)
main(puzzle_input)
