import sys
import numpy as np


def parse_args():
    if len(sys.argv) != 2:
        sys.exit('I need a file')
    return sys.argv[1]


def parse_input(filename):
    data = []
    with open(sys.argv[1]) as fd:
        for line in fd:
            s = line.strip().split("->")
            data.append([[int(x) for x in t.strip().split(',')] for t in s])
    return data


def create_bbox(data):
    bbox = [[1e8, 1e18], [0, 0]]
    for trace in data:
        for point in trace:
            bbox[0][1] = min(bbox[0][1], point[0])
            bbox[1][1] = max(bbox[1][1], point[0])
            bbox[0][0] = min(bbox[0][0], point[1])
            bbox[1][0] = max(bbox[1][0], point[1])
    return bbox


def sorted_range(a, b):
    if a > b:
        return range(b, a + 1)
    return range(a, b + 1)


def create_scene(data, start, with_floor):
    bbox = create_bbox(data)
    # if the 'floor' is present two more rows are required in the scene
    floor_offset = 2 if with_floor else 0
    # the j-index of the starting point is used to determine the maximum
    # columns that are required to represent the scene
    g = np.zeros((bbox[1][0] + floor_offset + 1,
                  bbox[1][1] + start[1] + 1), dtype=int)
    for trace in data:
        prev = trace[0]
        for pi in range(1, len(trace)):
            p = trace[pi]
            if p[0] == prev[0]:
                # vertical
                for i in sorted_range(p[1], prev[1]):
                    g[i, p[0]] = 1
            else:
                # horizontal
                assert p[1] == prev[1]
                for j in sorted_range(p[0], prev[0]):
                    g[p[1], j] = 1
            prev = p
    if with_floor:
        for j in range(0, g.shape[1]):
            g[g.shape[0] - 1, j] = 1
    return g, bbox


def problem_1(data, start):
    scene, bbox = create_scene(data, start, False)
    counter = 1
    while True:
        p = start
        lost = False
        while True:
            ni = p[0] + 1
            if ni > bbox[1][0]:
                lost = True
                break
            if scene[ni, p[1]] == 0:
                p = [ni, p[1]]
            elif scene[ni, p[1] - 1] == 0:
                p = [ni, p[1] - 1]
            elif scene[ni, p[1] + 1] == 0:
                p = [ni, p[1] + 1]
            else:
                scene[p[0], p[1]] = 2
                break
        if lost:
            break
        counter += 1
    return counter - 1


def problem_2(data, start):
    scene, bbox = create_scene(data, start, True)
    counter = 1
    while True:
        p = start
        while True:
            ni = p[0] + 1
            if scene[ni, p[1]] == 0:
                p = [ni, p[1]]
            elif scene[ni, p[1] - 1] == 0:
                p = [ni, p[1] - 1]
            elif scene[ni, p[1] + 1] == 0:
                p = [ni, p[1] + 1]
            else:
                scene[p[0], p[1]] = 2
                break
        # check if the starting point is filled
        if scene[start[0], start[1]] == 2:
            break
        counter += 1
    return counter


def main():
    filename = parse_args()
    data = parse_input(filename)
    start = (0, 500)
    print("Day 14, problem 1: ", problem_1(data, start))
    print("Day 14, problem 2: ", problem_2(data, start))


if __name__ == '__main__':
    main()
