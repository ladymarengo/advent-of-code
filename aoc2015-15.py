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


def making_recipe(ings, expected_volume):
    list_of_recipes = []
    recipe_lenght = len(ings)
    def adding(ings, expected_volume, actual_volume=0, recipe_dict={}):
        nonlocal list_of_recipes
        for ing in ings:
            for i in range(1, expected_volume):
                new_actual_volume = actual_volume + i
                new_recipe_dict = recipe_dict.copy()
                new_recipe_dict[i] = ing
                new_ings = list(ings)
                new_ings.remove(ing)
                if new_actual_volume < expected_volume:
                    adding(new_ings, expected_volume, new_actual_volume, new_recipe_dict)
                elif new_actual_volume == expected_volume and len(new_recipe_dict) == recipe_lenght:
                    list_of_recipes.append(new_recipe_dict.copy())
                    new_recipe_dict.clear()
                else:
                    break
    adding(ings, expected_volume)
    return list_of_recipes


def counting_scores(recipes_list):
    scores = []
    for recipe in recipes_list:
        total_capacity = 0
        total_durability = 0
        total_flavor = 0
        total_texture = 0
        total_calories = 0
        for amount in recipe:
            ing = recipe[amount]
            total_capacity += ing.capacity * amount
            total_durability += ing.durability * amount
            total_flavor += ing.flavor * amount
            total_texture += ing.texture * amount
            total_calories += ing.calories * amount
        if total_capacity < 0:
            total_capacity = 0
        if total_durability < 0:
            total_durability = 0
        if total_flavor < 0:
            total_flavor = 0
        if total_texture < 0:
            total_texture = 0
        if total_calories == 500:
            score = total_capacity * total_durability * total_flavor * total_texture
            scores.append(score)
    return scores


def main(raw_input, volume):
    list_of_ingredients = making_list_of_ingredients(raw_input)
    list_of_recipes = making_recipe(list_of_ingredients, volume)
    scores = counting_scores(list_of_recipes)
    return scores


scores = main(test_input, 100)
print(max(scores))
