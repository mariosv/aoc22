import sys

def score(c):
    if c.islower():
        return ord(c) - ord('a') + 1
    return ord(c) - ord('A') + 27


def problem_1():
    if len(sys.argv) != 2:
        sys.exit('I need a file')
    s = 0
    with open(sys.argv[1]) as fd:
        for line in fd:
            line = line.strip()
            n = len(line)
            h = int(n / 2)
            a = line[:h]
            b = line[h:]
            assert len(a) == len(b)
            for t in a:
                if t in b:
                    s += score(t)
                    break
    return s


def process(data):
    indices = []
    index = set()
    for i, d in enumerate(data):
        if 0 == i:
            for t in d:
                index.add(t)
        else:
            new_index = set()
            for t in d:
                if t in index:
                    new_index.add(t)
            index = new_index
    if len(index) != 1:
        raise Exception("Invalid input")
    return score(index.pop())


def problem_2():
    if len(sys.argv) != 2:
        sys.exit('I need a file')
    s = 0
    data = []
    with open(sys.argv[1]) as fd:
        for line in fd:
            line = line.strip()
            data.append(line)
            if len(data) == 3:
                s += process(data)
                data = []
    return s


def main():
    print("Day X, problem 1: ", problem_1())
    print("Day X, problem 2: ", problem_2())


if __name__ == '__main__':
    main()
