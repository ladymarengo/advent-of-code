import re

puzzle_input = open('aoc2020_22_input.txt').read().strip()


def main(text):
    instructions = re.search(r'Player 1:\n([\S\s]+)\nPlayer 2:\n([\S\s]+)', text)
    first_deck = [int(x) for x in instructions.group(1).strip().split('\n')]
    second_deck = [int(x) for x in instructions.group(2).strip().split('\n')]

    first_result = play(first_deck.copy(), second_deck.copy())
    answer = 0
    for e, card in enumerate(reversed(first_result)):
        answer += (e + 1) * card
    print(answer)

    result, deck = recursive_combat(first_deck, second_deck)
    second_answer = 0
    for e, card in enumerate(reversed(deck)):
        second_answer += (e+1) * card
    print(second_answer)


def play(first_deck, second_deck):
    count = 0

    while len(first_deck) > 0 and len(second_deck) > 0:
        count += 1
        cards = (first_deck.pop(0), second_deck.pop(0))
        if cards[0] > cards[1]:
            winner = first_deck
        else:
            winner = second_deck
        winner.append(max(cards))
        winner.append(min(cards))

    if len(first_deck) > len(second_deck):
        winner = first_deck
    else:
        winner = second_deck

    return winner


def recursive_combat(first_deck, second_deck):
    rounds = []

    while len(first_deck) > 0 and len(second_deck) > 0:
        if (first_deck,second_deck) in rounds:
            return 'first', first_deck
        rounds.append((first_deck.copy(), second_deck.copy()))
        cards = (first_deck.pop(0), second_deck.pop(0))

        if len(first_deck) >= cards[0] and len(second_deck) >= cards[1]:
            result, deck = recursive_combat(first_deck[:cards[0]], second_deck[:cards[1]])
            if result == 'first':
                winner = first_deck
            else:
                winner = second_deck

        elif cards[0] > cards[1]:
            winner = first_deck
        else:
            winner = second_deck

        if winner == first_deck:
            winner.append(cards[0])
            winner.append(cards[1])
        else:
            winner.append(cards[1])
            winner.append(cards[0])

    if len(first_deck) > len(second_deck):
        return 'first', first_deck
    else:
        return 'second', second_deck


main(puzzle_input)