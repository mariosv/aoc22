import sys
import numpy as np
import math
import functools


def parse_args():
    if len(sys.argv) != 2:
        sys.exit('I need a file')
    return sys.argv[1]


def parse_input(filename):
    data = []
    tmp = []
    with open(sys.argv[1]) as fd:
        for line in fd:
            s = line.strip()
            if len(s) == 0:
                assert len(tmp) == 2
                data.append(tmp)
                tmp = []
            else:
                tmp.append(eval(s))
        assert len(tmp) == 2
        data.append(tmp)
        tmp = []
    return data


def cmp(left, right):
    if isinstance(left, list) and isinstance(right, list):
        for i in range(len(left)):
            if i == len(right):
                return False
            c = cmp(left[i], right[i])
            if c is not None:
                return c
        if len(left) < len(right):
            return True
        return None  #check this
    elif isinstance(left, list) and not isinstance(right, list):
        return cmp(left, [right])
    elif not isinstance(left, list) and isinstance(right, list):
        return cmp([left], right)
    else:
        if left < right: return True
        elif left > right: return False
        return None
    assert(False)


def compare(left, right):
    c = cmp(left, right)
    if c is None:
        return 0
    elif c:
        return -1
    return 1


def problem_1(data):
    ok = []
    for i, p in enumerate(data):
        if compare(p[0], p[1]) < 1:
            ok.append(i + 1)
    return sum(ok)


def merge_data_and_dividers(data):
    d = []
    for left, right in data:
        d.append(left)
        d.append(right)
    d.append([[2,]])
    d.append([[6,]])
    return d


def problem_2(data):
    data = merge_data_and_dividers(data)
    data = sorted(data, key=functools.cmp_to_key(compare))
    a = data.index([[2]]) + 1
    b = data.index([[6]]) + 1
    return a * b


def main():
    filename = parse_args()
    data = parse_input(filename)
    print("Day 13, problem 1: ", problem_1(data))
    print("Day 13, problem 2: ", problem_2(data))


if __name__ == '__main__':
    main()
