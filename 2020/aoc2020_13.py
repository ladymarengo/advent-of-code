puzzle_input = open('aoc2020_13_input.txt').read().strip()

def first_puzzle(text):
    instructions = text.split('\n')
    timestamp = int(instructions[0])
    working_buses = [int(bus) for bus in instructions[1].split(',') if bus != 'x']
    waiting_times = [bus-(timestamp%bus) for bus in working_buses]
    min_time = min(waiting_times)
    print(working_buses[waiting_times.index(min_time)] * min_time)

def second_puzzle(text):
    instructions = text.split('\n')
    working_buses = [int(bus) for bus in instructions[1].split(',') if bus != 'x']
    buses = instructions[1].split(',')
    timestamp_dict = {}

    for e, bus in enumerate(buses):
        if bus != 'x':
            timestamp_dict[int(bus)] = e

    time = 0
    add_time = 1

    for e, bus in enumerate(working_buses[:-1]):
        add_time = add_time * bus
        next_bus = working_buses[e+1]
        while True:
            time += add_time
            if (time+timestamp_dict[next_bus])%next_bus == 0:
                break
        if e == len(working_buses) - 2:
            break

    print(time)


first_puzzle(puzzle_input)
second_puzzle(puzzle_input)