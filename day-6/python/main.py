import sys
from collections import deque

def check_unique(s):
    for i in range(len(s)):
        for j in range(i + 1, len(s)):
            if s[i] == s[j]:
                return False
    return True


def find_sequence(filename, length):
    n = 0
    last = deque()
    with open(filename) as fd:
        while True:
            n += 1
            char = fd.read(1)
            if not char:
                break
            last.append(char)
            if len(last) > length:
                last.popleft()
            unique = False
            if len(last) == length and check_unique(last):
                return n
    raise Excpetion("No unique sequence was found in the input")


def parse_args():
    if len(sys.argv) != 2:
        sys.exit('I need a file')
    return sys.argv[1]


def main():
    filename = parse_args()
    print("Day 6, problem 1: ", find_sequence(filename, 4))
    print("Day 6, problem 2: ", find_sequence(filename, 14))


if __name__ == '__main__':
    main()
