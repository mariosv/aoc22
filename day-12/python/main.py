import sys
import numpy as np
import math


def parse_args():
    if len(sys.argv) != 2:
        sys.exit('I need a file')
    return sys.argv[1]


def parse_input(filename):
    grid = []
    start = None
    end = None
    i = 0
    with open(sys.argv[1]) as fd:
        for line in fd:
            j = 0
            x = None
            row = []
            for c in line.strip():
                if c == 'S':
                    start = (i, j)
                    x = 0
                elif c == 'E':
                    end = (i, j)
                    x = ord('z') - ord('a')
                else:
                    x = ord(c) - ord('a')
                j += 1
                row.append(x)
            grid.append(row)
            i += 1
    assert start and end
    return grid, start, end


def idx(i, j, cols):
    return i * cols + j


def to_graph(grid, rows, cols):
    cmp = lambda other, cur: (other - cur) < 2
    g = np.zeros((rows * cols, rows * cols), dtype=int)
    for i in range(rows):
        for j in range(cols):
            v = grid[i][j]
            if i != 0 and cmp(grid[i - 1][j], v):
                g[idx(i, j, cols), idx(i - 1, j, cols)] = 1
            if i != (rows - 1) and cmp(grid[i + 1][j], v):
                g[idx(i, j, cols), idx(i + 1, j, cols)] = 1
            if j != 0 and cmp(grid[i][j - 1], v):
                g[idx(i, j, cols), idx(i, j - 1, cols)] = 1
            if j != (cols - 1) and cmp(grid[i][j + 1], v):
                g[idx(i, j, cols), idx(i, j + 1, cols)] = 1
    return g


def unvisited_with_min_distance(unvisited, dists):
    mind, mini = None, None
    i = 0
    for t in unvisited:
        if i == 0:
            mind = dists[t]
            mini = t
            i += 1
        else:
            if dists[t] < mind:
                mind = dists[t]
                mini = t
    return mini


def dijkstra(g, start):
    n = g.shape[0]
    # keep track of the unvisited nodes
    unvisited = set(range(n))
    # store distances in d
    d = np.zeros((n,), dtype=int) + int(1e8)
    d[start] = 0
    for i in range(n):
        u = unvisited_with_min_distance(unvisited, d)
        unvisited.remove(u)
        for v in range(n):
            if g[u, v] > 0 and (v in unvisited):
                d[v] = min(d[v], d[u] + g[u, v])
    return d


def problem_1(g, start, end):
    dists = dijkstra(g, start)
    return dists[end]


def problem_2(g, grid, end, cols):
    dists = dijkstra(np.transpose(g), end)
    return min([dists[idx(i, j, cols)] for ((i, j), el) in np.ndenumerate(grid) \
                if el == 0])


def main():
    # parse data
    filename = parse_args()
    grid, start, end = parse_input(filename)
    rows, columns = len(grid), len(grid[0])

    # create the graph
    g = to_graph(grid, rows, columns)
    start, end = idx(*start, columns), idx(*end, columns)

    print("Day 12, problem 1: ", problem_1(g, start, end))
    print("Day 12, problem 2: ", problem_2(g, grid, end, columns))


if __name__ == '__main__':
    main()
