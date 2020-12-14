import re

puzzle_input = open('aoc2020_14_input.txt').read().strip()

def main(text):
    instructions = text.split('\n')
    mem1 = {}
    mem2 = {}
    mask = ''

    def check_float(start, string, key_value):
        new_string = string
        nonlocal mask
        nonlocal list_mem_keys
        for i in range(start, len(key_value)):
            if mask[i] == '0':
                new_string += key_value[i]
                # print(i, new_string)
                if i == len(key_value)-1:
                    list_mem_keys.append(new_string)
                    return True
            elif mask[i] == '1':
                new_string += '1'
                # print(i, new_string)
                if i == len(key_value)-1:
                    list_mem_keys.append(new_string)
                    return True
            else:
                first_string = new_string + '0'
                second_string = new_string + '1'
                if i == len(key_value)-1:
                    list_mem_keys.append(first_string)
                    list_mem_keys.append(second_string)
                    return True
                else:
                    check_float(i+1, first_string, key_value)
                    check_float(i + 1, second_string, key_value)
                    return True

    for instruction in instructions:
        result = re.search(r'(.+) = (.+)', instruction)
        # print(result.group(1), result.group(2))
        if result.group(1) == 'mask':
            mask = result.group(2)
            # print(mask)
        else:
            mem_result = re.search(r'mem\[([0-9]+)\]', result.group(1))
            mem_key = int(mem_result.group(1))
            value = int(result.group(2))
            # print(mem_key, value)
            bin_value = "{:036b}".format(value)
            new_bin = ''

            for e, symbol in enumerate(bin_value):
                if mask[e] == 'X':
                    new_bin += symbol
                else:
                    new_bin+= mask[e]
            mem1[mem_key] = int(new_bin, 2)

            key_bin_value = "{:036b}".format(mem_key)
            # print(key_bin_value)
            list_mem_keys = []
            check_float(0, '', key_bin_value)
            for key in list_mem_keys:
                mem2[int(key, 2)] = value
            # print(mem2)

    sum1 = 0
    for key in mem1:
        sum1 += mem1[key]

    sum2 = 0
    for key in mem2:
        sum2 += mem2[key]
    print(sum2)


test_input = '''mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
mem[8] = 11
mem[7] = 101
mem[8] = 0'''

test_input2 = '''mask = 000000000000000000000000000000X1001X
mem[42] = 100
mask = 00000000000000000000000000000000X0XX
mem[26] = 1'''

main(test_input2)
main(puzzle_input)