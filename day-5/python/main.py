import sys
import copy


def parse_args():
    if len(sys.argv) != 2:
        sys.exit('I need a file')
    return sys.argv[1]


def parse_stacks(fd):
    stacks = [[] for _ in range(9)]
    for next_line in fd:
        line = next_line.rstrip()
        # read stacks
        if not line or line.lstrip()[0] != '[':
            break
        for i in range(9):
            index = 3 * i + i + 1
            if index < len(line):
                s = line[index]
                if s != ' ':
                    stacks[i].append(s)
    # reverse stacks
    for s in stacks:
        s = s.reverse()
    return stacks


def parse_moves(fd):
    moves = []
    for line in fd:
        s = line.split()
        if len(s) != 0:
            n = [int(t) for t in (s[1], s[3], s[5])]
            n[1] -= 1
            n[2] -= 1
            moves.append(n)
    return moves


def parse_input(filename):
    with open(filename) as fd:
        stacks = parse_stacks(fd)
        moves = parse_moves(fd)
    return stacks, moves


def problem_1(stacks, moves):
    for q, f, t in moves:
        for i in range(q):
            stacks[t].append(stacks[f].pop())
    return(''.join([s[-1] for s in stacks]))


def problem_2(stacks, moves):
    for q, f, t in moves:
        stacks[t].extend(stacks[f][-q:])
        stacks[f] = stacks[f][:-q]
    return(''.join([s[-1] for s in stacks]))


def main():
    filename = parse_args()
    stacks, moves = parse_input(filename)
    p1 = problem_1(copy.deepcopy(stacks), moves)
    p2 = problem_2(stacks, moves)

    print(f'Day 5, problem 1: {p1}')
    print(f'Day 5, problem 2: {p2}')


if __name__ == '__main__':
    main()
