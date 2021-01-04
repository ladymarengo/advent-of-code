import itertools

raw_input = '''11
30
47
31
32
36
3
1
5
3
32
36
15
11
46
26
28
1
19
3'''

list_input = raw_input.strip().split('\n')


def finding_combinations(containers, needed_volume):
    global_counter = 0

    def checking_combination(combination, needed_volume):
        volume = 0
        counter = 0
        nonlocal global_counter
        for container in combination:
            volume += int(container)
            if volume == needed_volume:
                counter += 1
                global_counter += 1
        return counter

    def making_combinations(containers, needed_volume):
        for i in range(2, len(containers)):
            combinations_list = list(itertools.combinations(containers, i))
            if len(combinations_list) > 0:
                number_of_combinations = 0
                for combination in combinations_list:
                    number_of_combinations += checking_combination(combination, needed_volume)
                if number_of_combinations > 0:
                    return number_of_combinations

    return global_counter, making_combinations(containers, needed_volume)



print(finding_combinations(list_input, 150))
