from dataclasses import dataclass
import operator

raw_input = open('2015-14-input.txt').read()
test_input = '''Comet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds.
Dancer can fly 16 km/s for 11 seconds, but then must rest for 162 seconds.'''

@dataclass
class Deer:
    deer_name: str
    speed: int
    fly_time: int
    rest_time: int
    fly_counter: int
    rest_counter: int
    points: int
    action: str
    distance: int


def splitting(string) -> Deer:
    lst = string.split(' ')
    return Deer(
        deer_name=lst[0],
        speed=int(lst[3]),
        fly_time=int(lst[6]),
        rest_time=int(lst[-2]),
        fly_counter=0,
        rest_counter=0,
        points=0,
        action='fly',
        distance=0
    )


def making_deers(raw_input):
    list_of_deers = list(map(splitting, raw_input.strip().split('\n')))
    return list_of_deers


def moving(deer):
    if deer.action == 'fly':
        deer.distance += deer.speed
        deer.fly_counter += 1
        if deer.fly_counter == deer.fly_time:
            deer.action = 'rest'
            deer.rest_counter = 0
    elif deer.action == 'rest':
        deer.rest_counter += 1
        if deer.rest_counter == deer.rest_time:
            deer.action = 'fly'
            deer.fly_counter = 0
    return deer.distance


def race(deers, finish_time):
    results = {}
    for sec in range(finish_time):
        places = {}
        for deer in deers:
            distance = moving(deer)
            places[distance] = deer
        winner_distance = max(places)
        for deer in deers:
            if deer.distance == winner_distance:
                deer.points += 1
    for deer in deers:
        results[deer.points] = deer.deer_name
    return results


def main(raw_input, finish_time):
    list_of_deers = making_deers(raw_input)
    results = race(list_of_deers, finish_time)
    return results


res = main(raw_input, 2503)
print(max(res))