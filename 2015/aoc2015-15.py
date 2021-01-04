from dataclasses import dataclass

test_input = '''Butterscotch: capacity -1, durability -2, flavor 6, texture 3, calories 8
Cinnamon: capacity 2, durability 3, flavor -2, texture -1, calories 3'''
real_input = '''Sprinkles: capacity 5, durability -1, flavor 0, texture 0, calories 5
PeanutButter: capacity -1, durability 3, flavor 0, texture 0, calories 1
Frosting: capacity 0, durability -1, flavor 4, texture 0, calories 6
Sugar: capacity -1, durability 0, flavor 0, texture 2, calories 8'''


@dataclass
class Ingredient:
    name: str
    capacity: int
    durability: int
    flavor: int
    texture: int
    calories: int


def making_ingredient(string) -> Ingredient:
    lst = string.split(' ')
    return Ingredient(
        name=lst[0].strip(':'),
        capacity=int(lst[2].strip(',')),
        durability=int(lst[4].strip(',')),
        flavor=int(lst[6].strip(',')),
        texture=int(lst[8].strip(',')),
        calories=int(lst[10]),
    )


def making_list_of_ingredients(raw_input):
    list_of_ingredients = list(map(making_ingredient, raw_input.strip().split('\n')))
    return list_of_ingredients


def making_amounts_for_2_ing(volume):
    list_of_amounts = [(x, y) for x in range(1,volume) for y in range(1,volume) if x + y == volume]
    return list_of_amounts


def making_amounts_for_4_ing(volume):
    list_of_amounts = [(x, y, z, w) for x in range(1,volume) for y in range(1,volume)
                       for z in range(1,volume) for w in range(1,volume) if x + y + z + w == volume]
    return list_of_amounts


def counting_scores_from_amounts(amounts_list, ing_list):
    scores = []
    for amounts in amounts_list:
        total_capacity = 0
        total_durability = 0
        total_flavor = 0
        total_texture = 0
        total_calories = 0
        for e, amount in enumerate(amounts):
            ing = ing_list[e]
            total_capacity += ing.capacity * amount
            total_durability += ing.durability * amount
            total_flavor += ing.flavor * amount
            total_texture += ing.texture * amount
            total_calories += ing.calories * amount
        if any(x <= 0 for x in [total_capacity, total_durability, total_flavor, total_texture]) or total_calories != 500:
            continue
        else:
            score = total_capacity * total_durability * total_flavor * total_texture
            scores.append(score)
    return scores


def main(raw_input, volume):
    list_of_ingredients = making_list_of_ingredients(raw_input)
    list_of_amounts = making_amounts_for_4_ing(volume)
    scores = counting_scores_from_amounts(list_of_amounts, list_of_ingredients)
    return scores


def test(raw_input, volume):
    list_of_ingredients = making_list_of_ingredients(raw_input)
    list_of_amounts = making_amounts_for_2_ing(volume)
    scores = counting_scores_from_amounts(list_of_amounts, list_of_ingredients)
    return scores


real_scores = main(real_input, 100)
test_scores = test(test_input, 100)
print(max(real_scores))
