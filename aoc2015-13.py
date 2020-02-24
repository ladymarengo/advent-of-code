raw_input = open('2015-13-input.txt').read()


def make_input(raw_input):
    list_input = raw_input.strip().split('\n')
    list_of_guests = []
    dict_of_happiness = {}
    for line in list_input:
        list_line = line.strip('.').split(' ')
        guest_name = list_line[0]
        happiness = int(list_line[3])
        neighbor = list_line[-1]
        command = list_line[2]
        if guest_name not in list_of_guests:
            list_of_guests.append(guest_name)
        if guest_name not in dict_of_happiness:
            dict_of_happiness[guest_name] = {}
        if command == 'gain':
            dict_of_happiness[guest_name][neighbor] = happiness
        if command == 'lose':
            dict_of_happiness[guest_name][neighbor] = 0 - happiness
    return list_of_guests, dict_of_happiness


def make_seating_arrangements(guests):
    list_of_arrangements = []
    def make_list(guests, arrangement=[]):
        nonlocal list_of_arrangements
        for guest in guests:
            new_arrangement = list(arrangement)
            new_arrangement.append(guest)
            new_list = list(guests)
            new_list.remove(guest)
            if len(new_list) > 0:
                make_list(new_list, new_arrangement)
            else:
                list_of_arrangements.append(list(new_arrangement))
                new_arrangement.clear()
    make_list(guests)
    return list_of_arrangements


def count_happiness(arrangements, happiness_dict):
    list_of_happiness = []
    for arrangement in arrangements:
        counter = 0
        for i in range(len(arrangement)):
            guest = happiness_dict[arrangement[i]]
            if i == len(arrangement) - 1:
                counter += guest[arrangement[0]]
                counter += guest[arrangement[i-1]]
            elif i == 0:
                counter += guest[arrangement[i+1]]
                counter += guest[arrangement[-1]]
            else:
                counter += guest[arrangement[i+1]]
                counter += guest[arrangement[i-1]]
        list_of_happiness.append(counter)
    return list_of_happiness


def main(raw_input):
    guest_list, guest_dict = make_input(raw_input)
    guest_list, guest_dict = adding_me(guest_list, guest_dict)
    list_of_arrangements = make_seating_arrangements(guest_list)
    list_of_happiness = count_happiness(list_of_arrangements, guest_dict)
    return(max(list_of_happiness))


def adding_me(guest_list, guest_dict):
    guest_dict['me'] = {}
    for guest in guest_list:
        guest_dict['me'][guest] = 0
        guest_dict[guest]['me'] = 0
    guest_list.append('me')
    return guest_list, guest_dict

max_happiness = main(raw_input)
print(max_happiness)