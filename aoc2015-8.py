import re

b = open('2015-8-input.txt').read().strip().split('\n')

counter_a = 0
counter_b = 0
counter_c = 0

for word in b:
    counter_a += len(word)
    new = re.sub(r'\\\"', 'a', word)
    new = re.sub(r'\\\\', 'a', new)
    new = re.sub(r"\\x[a-z0-9]{2}", 'a', new)
    lenght = len(new) - 2
    counter_b += lenght
    new2 = re.sub(r'\\\"', 'aaaa', word)
    new2 = re.sub(r'\\\\', 'aaaa', new2)
    new2 = re.sub(r"\\x[a-z0-9]{2}", 'aaaaa', new2)
    lenght_new = len(new2) + 4
    counter_c += lenght_new

counter = counter_a - counter_b
counter_new = counter_c - counter_a
print(counter, counter_new)



