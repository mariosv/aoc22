import sys


def parse_input(filename):
    data = []
    mode_b = False
    seq = None
    with open(sys.argv[1]) as fd:
        for line in fd:
            if len(line.strip()) == 0:
                mode_b = True
                continue
            if not mode_b:
                data.append(line.rstrip())
            else:
                seq = line
    return data, decode_seq(seq)


class Move(object):
    def __init__(self, v):
        self.steps = None
        self.rotate = None
        if isinstance(v, int):
            self.steps = v
        else:
            assert v == 'L' or v == 'R'
            self.rotate = v

    def __repr__(self):
        if self.steps is not None:
            return str(self.steps)
        return self.rotate


def print_maze(m, states):
    for i, row in enumerate(m):
        for j, el in enumerate(row):
            found = False
            for s in reversed(states):
                if i == s.position[0] and j == s.position[1]:
                    assert el == '.'
                    found = True
                    sys.stdout.write(str(s.direction))
                    break
            if not found:
                sys.stdout.write(el)
        print('')


def decode_seq(s):
    seq = []
    b = ""
    for c in s:
        if c.isalpha():
            if len(b) > 0:
                seq.append(Move(int(b)))
            assert c == 'L' or c == 'R'
            seq.append(Move(c))
            b = ""
        else:
            b += c
    if len(b) > 0:
        seq.append(Move(int(b)))
    return seq
