import re

puzzle_input = open('aoc2020_21_input.txt').read().strip()


def main(text):
    ingredients_set, allergens_dict = make_input(text)
    allergens = check_allergens(allergens_dict)
    good_ingredients = make_list_of_good_ingredients(ingredients_set, allergens)
    first_answer = count_appearences(good_ingredients, text)
    print(first_answer)
    second_answer = ','.join([allergens[k] for k in sorted(allergens)])
    print(second_answer)


def make_input(text):
    ingredients_set = set()
    allergens_dict = {}
    for line in text.split('\n'):
        res = re.search(r'(.+) \(contains (.+)\)', line)
        ingredients = res.group(1).split(' ')
        allergens = res.group(2).split(', ')
        ingredients_set.update(set(ingredients))
        for allergen in allergens:
            if not allergen in allergens_dict:
                allergens_dict[allergen] = [ingredients]
            else:
                allergens_dict[allergen].append(ingredients)
    return ingredients_set, allergens_dict


def check_allergens(allergens):
    allergens_probable_values = {}

    def check_ingredient(ingredient, allergens_dict):
        for line in allergens_dict:
            if not ingredient in line:
                return False
        return True

    for allergen in allergens:
        allergens_probable_values[allergen] = set()
        if len(allergens[allergen]) > 1:
            for ingredient in allergens[allergen][0]:
                if check_ingredient(ingredient, allergens[allergen][1:]):
                        allergens_probable_values[allergen].add(ingredient)
        else:
            allergens_probable_values[allergen] = set(allergens[allergen][0])

    def check_real_values(probable_values, real_values):
        changed = False
        for allergen in probable_values:
            if len(probable_values[allergen]) == 1:
                real_values[allergen] = list(probable_values[allergen])[0]
                changed = True
        return changed

    def update_probable_values(probable_values, real_values):
        for allergen in probable_values:
            for ingredient in list(probable_values[allergen]):
                if ingredient in real_values.values():
                    probable_values[allergen].remove(ingredient)

    allergen_real_values = {}
    change = True
    while change:
        change = False
        if check_real_values(allergens_probable_values, allergen_real_values):
            change = True
            update_probable_values(allergens_probable_values, allergen_real_values)

    return allergen_real_values


def make_list_of_good_ingredients(ingredients, allergens):
    good_ingredients = []
    for ingredient in ingredients:
        if not ingredient in allergens.values():
            good_ingredients.append(ingredient)
    return good_ingredients


def count_appearences(ingredients, text):
    sum = 0
    for ingredient in ingredients:
        sum += len(re.findall(fr'\b{ingredient}\b', text))
    return sum


main(puzzle_input)