import sys
import numpy as np


def parse_args():
    if len(sys.argv) != 2:
        sys.exit('I need a file')
    return sys.argv[1]


def parse_input(filename):
    data = []
    with open(filename) as fd:
        for line in fd:
            s = line.split()
            data.append([s[0], int(s[1])])
    return data


def move(m, i, j):
    if m == 'R':
        return (i, j + 1)
    if m == 'L':
        return (i, j - 1)
    if m == 'U':
        return (i + 1, j)
    return (i - 1, j)


def follow_dir(h, t):
    if (h - t) == 2:
        return t + 1
    if (t - h) == 2:
        return t - 1
    return t


def follow(hi, hj, ti, tj):
    if abs(hi - ti) < 2 and abs(hj -tj) < 2:
        return (ti, tj)

    nti = follow_dir(hi, ti)
    ntj = follow_dir(hj, tj)

    if abs(hi - ti) == 2 and abs(hj - tj) == 1:
        ntj = hj
    if abs(hi - ti) == 1 and abs(hj - tj) == 2:
        nti = hi

    return nti, ntj


def problem_1(data):
    ti, tj = 0, 0
    hi, hj = 0, 0
    pos = set()
    for m, s in data:
        for t in range(s):
            hi, hj = move(m, hi, hj)
            ti, tj = follow(hi, hj, ti, tj)
            pos.add((ti, tj))
    return len(pos)


def problem_2(data):
    nodes = []
    for i in range(10):
        nodes.append([0, 0])
    last_pos = set()
    for m, s in data:
        for t in range(s):
            nodes[0] = move(m, nodes[0][0], nodes[0][1])
            for j in range(1, len(nodes)):
                nodes[j] = follow(nodes[j - 1][0], nodes[j - 1][1],
                                  nodes[j][0], nodes[j][1])
                if j == len(nodes) - 1:
                    last_pos.add(nodes[j])
    return len(last_pos)


def main():
    filename = parse_args()
    data = parse_input(filename)
    print("Day 9, problem 1: ", problem_1(data))
    print("Day 9, problem 2: ", problem_2(data))


if __name__ == '__main__':
    main()
