from dataclasses import dataclass
import operator

raw_input = open('2015-14-input.txt').read()


@dataclass
class Deer:
    deer_name: str
    speed: int
    fly_time: int
    rest_time: int


def splitting(string) -> Deer:
    lst = string.split(' ')
    return Deer(
        deer_name=lst[0],
        speed=int(lst[3]),
        fly_time=int(lst[6]),
        rest_time=int(lst[-2]),
    )


def making_deers(raw_input):
    list_of_deers = list(map(splitting, raw_input.strip().split('\n')))
    return list_of_deers


def racing(deer, finish_time):
    finish = finish_time
    time = 0
    distance = 0
    def moving(deer, time, distance):
        nonlocal finish
        for sec in range(deer.fly_time):
            time += 1
            distance += deer.speed
            if time == finish:
                return time, distance
        for sec in range(deer.rest_time):
            time += 1
            if time == finish:
                return time, distance
        return time, distance
    while time != finish:
        time, distance = moving(deer, time, distance)
    return distance


def race_for_all(deers, finish_time):
    list_race = {}
    for deer in deers:
        list_race[deer.deer_name] = racing(deer, finish_time)
    return list_race

def main(raw_input, finish_time):
    list_of_deers = making_deers(raw_input)
    results = race_for_all(list_of_deers, finish_time)
    return results


res = main(raw_input, 2503)
winner = max(res.items(), key=operator.itemgetter(1))[0]
print('Winner is ', winner, ' with ', res[winner], ' distance.')