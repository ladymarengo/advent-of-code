import re
from dataclasses import dataclass

inform = open('2016_4_input.txt').read()


def summing_ids(real_list):
    sum_ids = 0
    for room in real_list:
        sum_ids += room.id
    return sum_ids


def checking_if_real(letters, checksum):
    letters_dict = {}
    for symbol in letters:
        if symbol not in letters_dict:
            letters_dict[symbol] = letters.count(symbol)
    if ''.join(sorted(letters_dict)[:5]) == checksum:
        return True


@dataclass
class Room:
    letters: str
    id: int
    checksum: str


def splitting(string) -> Room:
    res = re.search(r"([a-z-]+)-([0-9]+)\[([a-z]+)\]", string)
    return Room(
        letters=re.sub('-', '', res.group(1)),
        id=int(res.group(2)),
        checksum=res.group(3),
    )


def making_rooms(raw_input):
    list_of_all_rooms = list(map(splitting, raw_input.strip().split('\n')))
    return list_of_all_rooms


def making_list_of_real_rooms(lst):
    list_of_real_rooms = []
    for room in lst:
        if checking_if_real(room.letters, room.checksum):
            list_of_real_rooms.append(room)
    return list_of_real_rooms


def main(raw_input):
    list_of_rooms = making_rooms(raw_input)
    real_list = making_list_of_real_rooms(list_of_rooms)
    final_sum = summing_ids(real_list)
    print(final_sum)
    return final_sum

if __name__ == '__main__':
     assert main('''aaaaa-bbb-z-y-x-123[abxyz]
a-b-c-d-e-f-g-h-987[abcde]
not-a-real-room-404[oarel]
totally-real-room-200[decoy]''') == 1514
#      assert checking_if_real('aaaaabbbzyx', 'abxyz') == True
#      assert checking_if_real('totallyrealroom', 'decoy') != True
