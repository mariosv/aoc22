import sys
import copy
import math
from collections import deque


class Monkey(object):
    def __init__(self):
        self.items = None
        self.inspect = None
        self.throw = None
        self.activity = 0
        self.div = None


def parse_monkey(fd):
    m = Monkey()
    tst = None
    true = None
    false = None
    for line in fd:
        l = line.strip().replace(',', '')
        if l.startswith("Starting"):
            s = l.split()
            m.items = deque([int(t) for t in s[2:]])
        elif l.startswith("Operation"):
            f = l.split('=')[1]
            m.inspect = lambda old: eval(f)
        elif l.startswith("Test"):
            tst = int(l.split()[3])
            m.div = tst
        elif l.startswith("If true"):
            true = int(l.split()[5])
        elif l.startswith("If false"):
            false = int(l.split()[5])
        else:
            assert len(l) == 0
            break
    m.throw = lambda x: eval(f"{true} if (x % {tst} == 0) else {false}")
    return m


def parse_input():
    if len(sys.argv) != 2:
        sys.exit('I need a file')
    data = []
    with open(sys.argv[1]) as fd:
        for line in fd:
            if line.startswith("Monkey"):
                data.append(parse_monkey(fd))
            else:
                assert(False)
    return data


def common(monkeys, rounds, relax):
    for r in range(rounds):
        for m in monkeys:
            while True:
                try:
                    el = m.items.popleft()
                except:
                    break
                m.activity += 1
                el = m.inspect(el)
                el = relax(el)
                monkeys[m.throw(el)].items.append(el)
    activities = sorted([m.activity for m in monkeys])
    return activities[-2] * activities[-1]


def problem_1(monkeys):
    return common(monkeys, 20, lambda x: x // 3)


def problem_2(monkeys):
    # find "relax" ratio
    denom = math.prod([t.div for t in monkeys])
    return common(monkeys, 10000, lambda x: x % denom)


def main():
    data = parse_input()
    print("Day X, problem 1: ", problem_1(copy.deepcopy(data)))
    print("Day X, problem 2: ", problem_2(data))


if __name__ == '__main__':
    main()
