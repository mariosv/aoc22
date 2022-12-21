import sys
import numpy as np


def parse_args():
    if len(sys.argv) != 2:
        sys.exit('I need a file')
    return sys.argv[1]


class Monkey(object):
    def __init__(self, name, a, op=None, b=None, twisted=False):
        self.name = name
        self.a = a
        self.op = op
        self.b = b
        self.twisted = twisted

    def __repr__(self):
        if self.op is not None:
            return f"({self.name} = {self.a} {self.op} {self.b})"
        return f"({self.name} = {self.a})"

    def solve_for_a(self):
        assert self.op is not None
        op = None
        if self.name == 'root':
            return Monkey(self.a, self.b)
        if self.op == '+':
            op = '-'
        elif self.op == '-':
            op = '+'
        elif self.op == '*':
            op = '/'
        else:
            assert self.op == '/'
            op = '*'
        return Monkey(self.a, self.name, op, self.b)

    def solve_for_b(self):
        assert self.op is not None
        if self.name == 'root':
            return Monkey(self.b, self.a)
        if self.op == '+':
            return Monkey(self.b, self.name, '-', self.a)
        elif self.op == '-':
            return Monkey(self.b, self.a, '-', self.name, True)
        elif self.op == '*':
            return Monkey(self.b, self.name, '/', self.a)
        assert self.op == '/'
        return Monkey(self.b, self.a, '/', self.name, True)


def parse_input(filename):
    data = {}
    with open(sys.argv[1]) as fd:
        for line in fd:
            s = line.split(':')
            name = s[0]
            st = s[1].strip().split(' ')
            if len(st) == 1:
                data[name] = Monkey(name, int(st[0]))
            else:
                assert len(st) == 3
                data[name] = Monkey(name, st[0], st[1], st[2])
    return data


def eval_node(data, node_name):
    node = data[node_name]
    if node.op == None:
        return node.a
    a = eval_node(data, node.a)
    b = eval_node(data, node.b)
    op = node.op
    if node.op == '/':
        op = '//'
    return int(eval(f"{a} {node.op} {b}"))


def eval_unary_node(data, node_val):
    if isinstance(node_val, str):
        # root node case
        return eval_node(data, node_val)
    return node_val


def explore_binary_child(data, nn, evaluated, node_name):
    if nn.op is None:
        v = eval_unary_node(data, nn.a)
        evaluated[node_name] = v
        return v
    if nn.twisted:
        nb = reverse_eval_node(data, nn.b, evaluated)
        na = eval_node(data, nn.a)
    else:
        na = reverse_eval_node(data, nn.a, evaluated)
        nb = eval_node(data, nn.b)
    evaluated[nn.b] = nb
    v = int(eval(f"{na} {nn.op} {nb}"))
    evaluated[node_name] = v
    return v


def reverse_eval_node(data, node_name, evaluated):
    if node_name in evaluated:
        return evaluated[node_name]

    if node_name in data.keys() and node_name != 'humn':
        t = data[node_name]
        if t.op is None:
            evaluated[node_name] = t.a
            return t.a
    # find relevant entry
    for k, t in data.items():
        if t.a == node_name:
            nn = t.solve_for_a()
            return explore_binary_child(data, nn, evaluated, node_name)
        if t.b == node_name:
            nn = t.solve_for_b()
            return explore_binary_child(data, nn, evaluated, node_name)
    assert False


def problem_1(data):
    return eval_node(data, 'root')


def problem_2(data):
    evaluated = {}
    return reverse_eval_node(data, 'humn', evaluated)


def main():
    filename = parse_args()
    data = parse_input(filename)
    print("Day 21, problem 1: ", problem_1(data))
    print("Day 21, problem 2: ", problem_2(data))


if __name__ == '__main__':
    main()
