import sys


def ranges_overlap(a, b):
    if a[0] > b[1] or b[0] > a[1]:
        return False
    return True


def contains(a, b):
    if b[0] >= a[0] and b[1] <= a[1]:
        return True
    return False


def common(predicate):
    if len(sys.argv) != 2:
        sys.exit('I need a file')
    c = 0
    with open(sys.argv[1]) as fd:
        for line in fd:
            s = line.split(',')
            a = [int(t) for t in s[0].split('-')]
            b = [int(t) for t in s[1].split('-')]
            if predicate(a, b):
                c += 1
    return c


def problem_1_predicate(a, b):
    return contains(a, b) or contains(b, a)


def problem_2_predicate(a, b):
    return ranges_overlap(a, b)


def main():
    print("Day 4, problem 1: ", common(problem_1_predicate))
    print("Day 4, problem 2: ", common(problem_2_predicate))


if __name__ == '__main__':
    main()
