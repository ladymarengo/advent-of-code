import re
old_password = 'cqjxjnds'


def check_valid(string):
    if check_forbidden_letters(string) and check_pairs(string) and check_three_straight(string):
        return True


def check_forbidden_letters(string):
    forbiden_letters = ['i', 'o', 'l']
    for char in forbiden_letters:
        if char in string:
            return False
    return True


def check_pairs(string):
    res = re.findall(r"([a-z]{1})\1",string)
    if len(res) >= 2:
        return True


def check_three_straight(string):
    for i in range(len(string) - 2):
        if ord(string[i]) == ord(string[i+1]) - 1 and ord(string[i]) == ord(string[i+2]) - 2:
            return True


def increase(char, command):
    if ord(char) == 122:
        char = chr(97)
        command = True
    else:
        char = chr(ord(char) + 1)
        command = False
    return char, command


def increase_string(string):
    list_string = list(string)
    have_to_wrap = True
    for i in range(len(list_string) - 1, -1, -1):
        if have_to_wrap:
            list_string[i], have_to_wrap = increase(list_string[i], have_to_wrap)
    return(''.join(list_string))


def change_password(password):
    new_password = password
    new_password = increase_string(new_password)
    while not check_valid(new_password):
        new_password = increase_string(new_password)
    return(new_password)


new_password = change_password(old_password)
print(new_password)
print(change_password(new_password))
