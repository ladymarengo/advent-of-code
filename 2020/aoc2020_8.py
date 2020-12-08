import re

def create_instr(text):
    instructions = []
    for line in text.split('\n'):
        res = re.search(r"(.+) (.+)", line)
        instructions.append([res.group(1), int(res.group(2))])
    return instructions

def execute(instructions):
    done_instr = []
    acc = 0
    execute_now = 0
    while execute_now not in done_instr:
        if execute_now == len(instructions):
            print(acc)
            return True
        done_instr.append(execute_now)
        command = instructions[execute_now][0]
        number = instructions[execute_now][1]
        if command == 'acc':
            acc += number
            execute_now += 1
        elif command == 'nop':
            execute_now += 1
        elif command == 'jmp':
            execute_now += number
    print(acc)
    return None

def change_list(instr, index):
    new_list = []
    for e, line in enumerate(instr):
        new_line = line.copy()
        if e == index and instr[index][0] == 'nop':
            new_line[0] = 'jmp'
        elif e == index and instr[index][0] == 'jmp':
            new_line[0] = 'nop'
        new_list.append(new_line)
    return new_list


def main(text):
    instructions = create_instr(text)
    execute(instructions)
    for index in range(len(instructions)-1):
        new_instructions = change_list(instructions, index)
        if execute(new_instructions):
            print('Terminated')
            break


puzzle_input = open('aoc2020_8_input.txt').read().strip()

test_input = '''nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6'''


main(test_input)
main(puzzle_input)