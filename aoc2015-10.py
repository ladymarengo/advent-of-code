from copy import copy
new_string = '3113322113'
counter = 1
for i in range(50):
    first_string = copy(new_string)
    new_string = ''
    for i in range(len(first_string)):
        if i == len(first_string) - 1:
            new_string += str(counter)
            new_string += str(first_string[i])
        elif first_string[i] == first_string[i + 1]:
            counter += 1
        else:
            counter_append = copy(counter)
            new_string += str(counter)
            new_string += str(first_string[i])
            counter = 1

print(len(new_string))
