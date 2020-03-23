import aoc2015_19

def test_task():
    test_input = '''H => HO
H => OH
O => HH

HOH'''

    replacements, keys, molecule = aoc2015_19.make_input(test_input)

    assert aoc2015_19.make_replacements(replacements, keys, molecule) == 4
