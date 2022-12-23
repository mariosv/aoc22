import sys

import data
import problem_1
import problem_2


def parse_args():
    if len(sys.argv) != 2:
        sys.exit('I need a file')
    return sys.argv[1]


def main():
    filename = parse_args()
    maze, moves = data.parse_input(filename)
    print("Day 22, problem 1: ", problem_1.solve(maze, moves))
    print("Day 22, problem 2: ", problem_2.solve(maze, moves))


if __name__ == '__main__':
    main()
