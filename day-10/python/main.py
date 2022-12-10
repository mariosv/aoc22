import sys


def parse_args():
    if len(sys.argv) != 2:
        sys.exit('I need a file')
    return sys.argv[1]


def parse_input(filename):
    data = []
    with open(filename) as fd:
        for line in fd:
            s = line.split()
            cmd = s[0]
            if len(s) == 2:
                arg = int(s[1])
                data.append((cmd, arg))
            else:
                data.append((cmd,))
    return data


def problem_1(data):
    wait = False
    acc = 1
    i = 0
    s = 0
    toadd = 0
    for cycle in range(1, 221):
        if cycle in (20, 60, 100, 140, 180, 220):
            s += (cycle * acc)
        if wait:
            wait = False
            acc += to_add
            continue
        assert i < len(data)
        cmd = data[i]
        i += 1
        if len(cmd) == 2:
            to_add = cmd[1]
            wait = True
    return s


def print_pixel(screen_pos, mid):
    sx = screen_pos % 40
    sy = screen_pos / 40
    if sx == 0:
        sys.stdout.write('\n')
    if sx in (mid - 1, mid, mid + 1):
        sys.stdout.write('#')
    else:
        sys.stdout.write('.')


def problem_2(data):
    wait = False
    acc = 1
    i = 0
    toadd = 0
    screen_pos = 0
    cycle = 1
    while i < len(data):
        print_pixel(screen_pos, acc)
        screen_pos += 1
        if wait:
            wait = False
            acc += to_add
            cycle += 1
            continue
        assert i < len(data)
        cmd = data[i]
        i += 1
        if len(cmd) == 2:
            to_add = cmd[1]
            wait = True
        cycle += 1
    print()


def main():
    filename = parse_args()
    data = parse_input(filename)
    print("Day X, problem 1: ", problem_1(data))
    print("Day X, problem 2: ")
    problem_2(data)



if __name__ == '__main__':
    main()
