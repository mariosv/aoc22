import sys
import numpy as np

def parse_input():
    if len(sys.argv) != 2:
        sys.exit('I need a file')
    data = []
    with open(sys.argv[1]) as fd:
        for line in fd:
            row = [int(t) for t in line.strip()]
            data.append(row)
    return np.array(data)


def nearest_obstacle(block, v):
    """ Check if the array block "hides" v
    """
    i = 0
    hides = False
    for obstacle in block:
        i += 1
        if v <= obstacle:
            hides = True
            break
    return i, hides


def candidate_ranges(data, row, column, n):
    return [[range(row - 1, -1, -1), column],  # bottom
            [range(row + 1, n), column], # top
            [row, range(column - 1, -1, -1)], # left
            [row, range(column + 1, n)]] # right


def check_visibility(data, row, column, n):
    # boundary nodes
    if row == 0 or column == 0 or row == n - 1 or column == n - 1:
        return True
    # inner nodes
    v = data[row, column]
    r = True
    for cr in candidate_ranges(data, row, column, n):
        if not nearest_obstacle(data[cr[0], cr[1]], v)[1]:
            return True
    return False


def problem_1(data):
    n = len(data)
    count = 0
    for i in range(n):
        for j in range(n):
            if check_visibility(data, i, j, n):
                count += 1
    return count


def count_neighbors(data, row, column, n):
    v = data[row, column]
    r = 1
    for cr in candidate_ranges(data, row, column, n):
         r *= nearest_obstacle(data[cr[0], cr[1]], v)[0]
    return r


def problem_2(data):
    n = len(data)
    r = 0
    for i in range(n):
        for j in range(n):
            r = max(r, count_neighbors(data, i, j, n))
    return r


def main():
    data = parse_input()
    print("Day 8, problem 1: ", problem_1(data))
    print("Day 8, problem 2: ", problem_2(data))


if __name__ == '__main__':
    main()
