import sys
import numpy as np


def construct_cube(maze, n, sections):
    """
    Interprets the input maze into cube faces. A standard indexing convention
    is kept. The first face encountered on the first maze line is always face
    0 and the other face ids are inferred from their relative position.
    Forward and backward mappings between the two representations of the input
    are returned.
    """
    to_maze = np.zeros((6, n, n, 2), dtype=int)
    face_rotations = [0] * 6
    mapping = Cube_map()
    cube = np.zeros((6, n, n), dtype=int)
    # find start
    for j, el in enumerate(maze[0]):
        if el != ' ':
            start = (0, j)
            break
    faces_found = set()
    face_starts = [[None, None] for _ in range(6)]
    stack = []
    stack.append((start, Rotated_face(0, 0)))
    while len(stack) != 0:
        p, rf = stack.pop()
        faces_found.add(rf.face)
        read_into_face(maze, cube, rf, p, n, to_maze)
        face_rotations[rf.face] = rf.rot
        # check right
        if p[1] + n < len(maze[p[0]]):
            nf = mapping.right_of[rf.rot][rf.face]
            if nf[0] not in faces_found:
                stack.append(((p[0], p[1] + n), Rotated_face(*nf)))
        # check left
        if p[1] != 0 and maze[p[0]][p[1] - 1] != ' ':
            assert p[1] - n >= 0 and maze[p[0]][p[1] - n] != ' '
            nf = mapping.left_of[rf.rot][rf.face]
            if nf[0] not in faces_found:
                stack.append(((p[0], p[1] - n), Rotated_face(*nf)))
        # check bottom
        if p[0] + n < len(maze) \
           and len(maze[p[0] + n]) > p[1] \
           and maze[p[0] + n][p[1]] != ' ':
            nf = mapping.below_of[rf.rot][rf.face]
            if nf[0] not in faces_found:
                stack.append(((p[0] + n, p[1]), Rotated_face(*nf)))
        # check top
        if p[0] != 0 \
           and len(maze[p[0] - 1]) > p[1] \
           and maze[p[0] - 1][p[1]] != ' ':
            assert p[0] - n >= 0 and maze[p[0] - n][p[1]] != ' '
            nf = mapping.above_of[rf.rot][rf.face]
            if nf[0] not in faces_found:
                stack.append(((p[0] - n, p[1]), Rotated_face(*nf)))
    return cube, to_maze, mapping, face_rotations


class Rotated_face(object):
    def __init__(self, face, rot):
        self.face = face
        self.rot = rot # times rotated ccw

    def __str__(self):
        return f"{self.face}rot{self.rot}ccw"


class Cube_map(object):
    def __init__(self):
        self.right_of = [
            [[3, 0], [0, 0], [1, 0], [2, 0], [3, 3], [3, 1]], # ccw: 0
            [[4, 1], [4, 0], [4, 3], [4, 2], [2, 3], [0, 1]], # ccw: 1
            [[1, 2], [2, 2], [3, 2], [0, 2], [1, 3], [1, 1]], # ccw: 2
            [[5, 3], [5, 0], [5, 1], [5, 2], [0, 3], [2, 1]]  # ccw: 3
        ]
        self.left_of = [
            [[1, 0], [2, 0], [3, 0], [0, 0], [1, 1], [1, 3]], # ccw: 0
            [[5, 1], [5, 2], [5, 3], [5, 0], [0, 1], [2, 3]], # ccw: 1
            [[3, 2], [0, 2], [1, 2], [2, 2], [3, 1], [3, 3]], # ccw: 2
            [[4, 3], [4, 2], [4, 1], [4, 0], [2, 1], [0, 3]], # ccw: 3
        ]
        self.below_of = [
            [[4, 0], [4, 3], [4, 2], [4, 1], [2, 2], [0, 0]], # ccw: 0
            [[1, 1], [2, 1], [3, 1], [0, 1], [1, 2], [1, 0]], # ccw: 1
            [[5, 2], [5, 3], [5, 0], [5, 1], [0, 2], [2, 0]], # ccw: 2
            [[3, 3], [0, 3], [1, 3], [2, 3], [3, 2], [3, 0]], # ccw: 3
        ]
        self.above_of = [
            [[5, 0], [5, 1], [5, 2], [5, 3], [0, 0], [2, 2]], # ccw: 0
            [[3, 1], [0, 1], [1, 1], [2, 1], [3, 0], [3, 2]], # ccw: 1
            [[4, 2], [4, 1], [4, 0], [4, 3], [2, 0], [0, 2]], # ccw: 2
            [[1, 3], [2, 3], [3, 3], [0, 3], [1, 0], [1, 2]] # ccw: 3
        ]
        # check
        for rot in range(len(self.right_of)):
            for f in range(len(self.right_of[rot])):
                e = self.right_of[rot][f]
                assert self.left_of[e[1]][e[0]][0] == f
                assert self.left_of[e[1]][e[0]][1] == rot
        for rot in range(len(self.below_of)):
            for f in range(len(self.below_of[rot])):
                e = self.below_of[rot][f]
                assert self.above_of[e[1]][e[0]][0] == f
                assert self.above_of[e[1]][e[0]][1] == rot


def read_into_face(maze, cube, rotated_face, start, n, to_maze):
    face = rotated_face.face
    rot = rotated_face.rot
    for i in range(n):
        for j in range(n):
            ni = i
            nj = j
            if rot == 1:
                ni = j
                nj = n - 1 - i
            elif rot == 2:
                ni = n - 1 - i
                nj = n - 1 - j
            elif rot == 3:
                ni = n - 1 - j
                nj = i
            else:
                assert rot == 0
            cube[face, ni, nj] = to_int(maze[start[0] + i][start[1] + j])
            to_maze[face, ni, nj, 0] = start[0] + i
            to_maze[face, ni, nj, 1] = start[1] + j


def to_int(c):
    if c == '.':
        return 0
    assert c == '#'
    return 1


def decide_cube_n(maze):
    lines = len(maze)
    cols = []
    prev_start = None
    prev_end = None
    for row in maze:
        start = None
        for j, t in enumerate(row):
            if t == ' ':
                continue
            start = j
            break
        end = len(row)
        if prev_start is None:
            prev_start = start
        else:
            if start != prev_start:
                cols.append(prev_end - prev_start)
                prev_start = start
                prev_end = end
                continue
        if prev_end is None:
            prev_end = end
        else:
            if end != prev_end:
                cols.append(prev_end - prev_start)
                prev_start = start
                prev_end = end
                continue
    cols.append(prev_end - prev_start)
    n = min(cols)
    assert sum(cols) // n == 6
    assert lines == n * len(cols)
    return n, len(cols)


def print_cube(cube):
    n = cube.shape[1]
    assert n == cube.shape[2]
    c = lambda x: '.' if x == 0 else '#'
    for i in range(n):
        for f in range(6):
            for j in range(n):
                sys.stdout.write(c(cube[f, i, j]))
            sys.stdout.write('  ')
        print()
