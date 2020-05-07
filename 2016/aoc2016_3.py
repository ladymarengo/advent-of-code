import re

list_of_triangles = open('2016_3_input.txt').read()


def making_list_horizontally(raw_input):
    tr_list = []
    for triangle in raw_input.split('\n'):
        res = re.search(r"\s+([0-9]+)\s+([0-9]+)\s+([0-9]+)", triangle)
        tr = [int(res.group(1)), int(res.group(2)), int(res.group(3))]
        tr_list.append(tr)
    return tr_list


def checking_one_triangle(a, b, c):
    if a + b > c and a + c > b and c + b > a:
        return True


def checking_triangles_from_list_horizontally(raw_input):
    tr_list = making_list_horizontally(raw_input)
    count = 0
    for triangle in tr_list:
        if checking_one_triangle(triangle[0], triangle[1], triangle[2]):
            count += 1
    return count


def making_list_vertically(raw_input):
    tr_list = []
    first_column = []
    second_column = []
    third_column = []
    for triangle in raw_input.split('\n'):
        res = re.search(r"\s+([0-9]+)\s+([0-9]+)\s+([0-9]+)", triangle)
        first_column.append(int(res.group(1)))
        second_column.append(int(res.group(2)))
        third_column.append(int(res.group(3)))
    tr_list.append(first_column)
    tr_list.append(second_column)
    tr_list.append(third_column)
    return tr_list


def checking_triangles_from_list_vertically(raw_input):
    tr_list = making_list_vertically(raw_input)
    count = 0
    for column in tr_list:
        for i in range(0, len(column)-2, 3):
            if checking_one_triangle(column[i], column[i+1], column[i+2]):
                count += 1
    return count


if __name__ == '__main__':
     # assert making_list('''  566  477  376
  # 575  488  365''') == [[566, 477, 376], [575, 488, 365]]
     print(checking_triangles_from_list_horizontally(list_of_triangles))
     print(checking_triangles_from_list_vertically(list_of_triangles))