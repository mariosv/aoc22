import sys
from enum import Enum
import numpy as np


class Choice(Enum):
    Rock = 0
    Paper = 1
    Scissors = 2


class Outcome(Enum):
    Lose = 0
    Draw = 1
    Win = 2


def score(you: Choice, outcome: Outcome) -> int:
    return you.value + 1 + outcome.value * 3


class Game(object):
    def __init__(self):
        L, D, W = [t.value for t in list(Outcome)]
        # codify the rules of the game in a matrix where rows and columns
        # represent the choices of you and the oponent respectively
        self._rules = np.array([[D, L, W],
                                [W, D, L],
                                [L, W, D]])

    def evaluate_round(self, you: Choice, other: Choice) -> Outcome:
        return Outcome(self._rules[you.value, other.value])

    def evaluate_round_corrected(self,
                                 other: Choice,
                                 outcome: Outcome) -> Choice:
        row = np.where(self._rules[:, other.value] == outcome.value)[0][0]
        return Choice(row)


def parse_code(c):
    if c == 'A' or c == 'X':
        return 0
    if c == 'B' or c == 'Y':
        return 1
    assert c == 'C' or c == 'Z'
    return 2


def parse_input(filename):
    rounds = []
    with open(filename) as fd:
        for line in fd:
            s = line.split()
            rounds.append([parse_code(t) for t in s])
    return rounds


def parse_args():
    if len(sys.argv) != 2:
        sys.exit('Invalid arguments: I need the input file')
    return sys.argv[1]


def problem_1(g, rounds):
    total_score = 0
    for c1, c2 in rounds:
        other, you = Choice(c1), Choice(c2)
        outcome = g.evaluate_round(you, other)
        total_score += score(you, outcome)
    return total_score


def problem_2(g, rounds):
    total_score = 0
    for c1, c2 in rounds:
        other, outcome = Choice(c1), Outcome(c2)
        you = g.evaluate_round_corrected(other, outcome)
        total_score += score(you, outcome)
    return total_score


def main():
    filename = parse_args()
    rounds = parse_input(filename)
    g = Game()
    print("Day 2, problem 1: ", problem_1(g, rounds))
    print("Day 2, problem 2: ", problem_2(g, rounds))


if __name__ == '__main__':
    main()
