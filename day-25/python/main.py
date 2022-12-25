import sys
import numpy as np
import math

pows = []

def parse_args():
    if len(sys.argv) != 2:
        sys.exit('I need a file')
    return sys.argv[1]


def parse_input(filename):
    data = []
    t = { '2': 2,
          '1': 1,
          '0': 0,
          '-': -1,
          '=': -2
    }
    with open(sys.argv[1]) as fd:
        for line in fd:
            num = []
            for c in line.strip():
                assert c in t
                num.append(t [c])
            data.append(num)
    return data


def snafu_to_dec(n):
    s = 0
    i = 0
    for d in reversed(n):
        if i == len(pows):
            pows.append(5 ** i)
        s += (d * pows[i])
        i += 1
    return s


def concat(r, nr):
    assert len(r) > 0
    mid = []
    t = r[-1][0]
    while t > nr[0][0] + 1:
        mid.append([r[-1][0] - 1, 0])
        t -= 1
    return r + mid + nr


def find_top_place(n):
    i = 0
    while True:
        if i == len(pows):
            pows.append(5 ** i)
        prev = 0
        for t in range(i):
            prev += 2 * pows[t]
        if n > (2 * pows[i]) + prev:
            i += 1
        else:
            break
    return i


def calc_value_for_digit(n, i):
    d = 0
    while d < 4:
        if i == 0:
            if n <= pows[i] * d:
                break
        else:
            prev = 0
            for t in range(i):
                prev += 2 * pows[t]
            if n <= pows[i] * d + prev:
                break
        d += 1
    assert d != 3


def dec_to_snafu_lst(n):
    i = find_top_place(n)
    d = calc_value_for_digit(n, i)
    r = []
    r.append([i, d])
    nn = d * pows[i]
    if i == 0 and n - nn == 0:
        return r
    if n > nn:
        nr = dec_to_snafu_lst(n - nn)
        return concat(r, nr)
    nr = dec_to_snafu_lst(nn - n)
    for j in range(len(nr)):
        nr[j][1] = -nr[j][1]
    return concat(r, nr)


def dec_to_snafu(n):
    return [t[1] for t in dec_to_snafu_lst(n)]


def encode(snafu):
    m = {1: '1', 2: '2', 0: '0', -1: '-', -2: '='}
    s = ""
    for t in snafu:
        s += m[t]
    return s


def problem_1(data):
    s = 0
    for d in data:
        s += snafu_to_dec(d)
    return  encode(dec_to_snafu(s))


def main():
    filename = parse_args()
    data = parse_input(filename)
    print("Day 25, problem 1: ", problem_1(data))
    print("Season's greetings!")


if __name__ == '__main__':
    main()
