import sys
import numpy as np


def parse_args():
    if len(sys.argv) != 2:
        sys.exit('I need a file')
    return sys.argv[1]


def parse_input(filename):
    data = []
    with open(sys.argv[1]) as fd:
        for line in fd:
            s = line.replace(',', '').replace(':', '').split()
            assert len(s) == 10
            sx = int(s[2].split('=')[1])
            sy = int(s[3].split('=')[1])
            bx = int(s[8].split('=')[1])
            by = int(s[9].split('=')[1])
            data.append([[sx, sy], [bx, by]])
    return data


def dist(a, b):
    return abs(a[0] - b[0]) + abs(a[1] - b[1])


def problem_1(data, column):
    excl = set()
    res = set()
    for s, b in data:
        if b[1] == column:
            res.add(b[0])
        if s[1] == column:
            res.add(s[0])
        yd = abs(s[1] - column)
        dd = dist(s, b)
        if yd > dd: continue
        xd = dd - yd
        for x in range(s[0] - xd, s[0] + xd + 1):
            if x not in res:
                excl.add(x)
    return len(excl)


def find_perimeter(s, d):
    # left to top
    xr = list(range(s[0] - d - 1, s[0]))
    yr = list(range(s[1], s[1] - d - 1, -1))
    r = list(zip(xr, yr))
    # top to right
    xr = list(range(s[0], s[0] + d + 1))
    yr = list(range(s[1] - d - 1, s[1]))
    r += list(zip(xr, yr))
    # right to bottom
    xr = list(range(s[0] + d + 1, s[0], -1))
    yr = list(range(s[1], s[1] + d + 1))
    r += list(zip(xr, yr))
    # bottom to left
    xr = list(range(s[0], s[0] - d - 1, -1))
    yr = list(range(s[1] + d + 1, s[1], - 1))
    r += list(zip(xr, yr))
    return r


def problem_2(data, max_dim):
    # find dists per sensor
    sensor_radius = [dist(*t) for t in data]
    for i, s in enumerate(data):
        d = sensor_radius[i]
        per = find_perimeter(s[0], d)
        for p in per:
            if not (p[0] in range(max_dim) and p[1] in range(max_dim)):
                continue
            found = False
            for j, ns in enumerate(data):
                if i == j:
                    continue
                nd = dist(p, ns[0])
                if nd <= sensor_radius[j]:
                    found = True
                    break
            if not found:
                return p[0] * 4000000 + p[1]
    assert False


def main():
    filename = parse_args()
    data  = parse_input(filename)
    print("Day 15, problem 1: ", problem_1(data, 2000000))
    print("Day 15, problem 2: ", problem_2(data, 4000000))


if __name__ == '__main__':
    main()
