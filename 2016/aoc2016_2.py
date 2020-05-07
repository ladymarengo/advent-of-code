instructions = open('2016_2_input.txt').read()


def following_instructions(instr):
    keypad = [['1', '2', '3'], ['4', '5', '6'], ['7', '8', '9']]
    x, y = 1, 1
    code = ''
    for instruction in instr.split('\n'):
        for symbol in instruction:
            if symbol == 'U' and y > 0:
                y -= 1
            elif symbol == 'D' and y < 2:
                y += 1
            elif symbol == 'R' and x < 2:
                x += 1
            elif symbol == 'L' and x > 0:
                x -= 1
        code += keypad[y][x]
    return code


def following_instructions_second_part(instr):
    keypad = [['0', '0', '0', '0', '0', '0', '0'],
              ['0', '0', '0', '1', '0', '0', '0'],
              ['0', '0', '2', '3', '4', '0', '0'],
              ['0', '5', '6', '7', '8', '9', '0'],
              ['0', '0', 'A', 'B', 'C', '0', '0'],
              ['0', '0', '0', 'D', '0', '0', '0'],
              ['0', '0', '0', '0', '0', '0', '0']]
    x, y = 1, 3
    code = ''
    for instruction in instr.split('\n'):
        for symbol in instruction:
            if symbol == 'U' and keypad[y-1][x] != '0':
                y -= 1
            elif symbol == 'D' and keypad[y+1][x] != '0':
                y += 1
            elif symbol == 'R' and keypad[y][x+1] != '0':
                x += 1
            elif symbol == 'L' and keypad[y][x-1] != '0':
                x -= 1
        code += keypad[y][x]
    return code

if __name__ == '__main__':
     assert following_instructions('''ULL
RRDDD
LURDL
UUUUD''') == '1985'
     assert following_instructions_second_part('''ULL
RRDDD
LURDL
UUUUD''') == '5DB3'
     print(following_instructions_second_part(instructions))