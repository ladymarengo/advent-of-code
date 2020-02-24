cities_input = open('2015-9-input.txt').read().strip().split('\n')


def make_input(raw_input):
    cities_dict = {}
    cities_list = []
    for string in raw_input:
        distance_list = string.split(' ')
        if distance_list[0] not in cities_list:
            cities_list.append(distance_list[0])
        if distance_list[2] not in cities_list:
            cities_list.append(distance_list[2])
        if distance_list[0] not in cities_dict:
            cities_dict[distance_list[0]] = {}
        if distance_list[2] not in cities_dict:
            cities_dict[distance_list[2]] = {}
        cities_dict[distance_list[0]][distance_list[2]] = int(distance_list[4])
        cities_dict[distance_list[2]][distance_list[0]] = int(distance_list[4])
    return cities_dict, cities_list


def making_list_of_routes(list_of_cities):
    list_of_routes = []
    def make_route(list_of_cities, route=[]):
        nonlocal list_of_routes
        for city in list_of_cities:
            new_route = list(route)
            new_route.append(city)
            new_list = list(list_of_cities)
            new_list.remove(city)
            if len(new_list) > 0:
                make_route(new_list, new_route)
            else:
                list_of_routes.append(list(new_route))
                new_route.clear()
    make_route(list_of_cities)
    return list_of_routes


def making_list_of_distances(routes, dict_of_cities):
    list_of_distances = []
    def counting_distance(routes, dict_of_cities):
        nonlocal list_of_distances
        for route in routes:
            distance = 0
            for i in range(len(route) - 1):
                distance += dict_of_cities[route[i]][route[i + 1]]
            list_of_distances.append(distance)
    counting_distance(routes, dict_of_cities)
    return list_of_distances


def get_min_and_max_distance(raw_input):
    cities_dict, cities_list = make_input(raw_input)
    list_of_route = making_list_of_routes(cities_list)
    list_of_distance = making_list_of_distances(list_of_route, cities_dict)
    print('minimum distance is' , min(list_of_distance), '\nmaximum distance is', max(list_of_distance))


get_min_and_max_distance(cities_input)

