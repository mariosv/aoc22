import numpy as np

from state import (State, rotate)
from cube import (construct_cube, decide_cube_n, print_cube)
from direction import Direction


def solve(maze, moves):
    n, sections = decide_cube_n(maze)
    cube, to_maze, cube_map, face_rotations = construct_cube(maze, n, sections)
    state = find_start(cube)
    states = []
    states.append(state)
    for m in moves:
        if m.rotate is not None:
            state = rotate(state, m.rotate)
            states.append(state)
        else:
            assert m.steps
            for _ in range(m.steps):
                nstate = try_move(cube, state, cube_map)
                if nstate is None:
                    break
                state = nstate
                states.append(state)
    final = states[-1].position
    final_dir = states[-1].direction
    # translate to maze-coordinates
    mi, mj = to_maze[final[0], final[1], final[2],:]
    # translate to maze-direction
    md = translate_dir_on_face(final_dir, face_rotations[final[0]])
    # calculate the "password"
    return 1000 * (mi + 1) + 4 * (mj + 1) + md.value


def find_start(cube):
    """ The starting point is the "leftmost open tile of the top row of tiles"
    which in this case translates to the first zero element on the first row
    of face 0
    """
    n = cube.shape[1]
    face = 0
    row = 0
    for j in range(n):
        if cube[face, row, j] == 0:
            return State((face, row, j), Direction.Right)


def rot_ccw(i, j, n):
    ni = n - 1 - j
    nj = i
    return ni, nj


def translate(i, j, n, d):
    ni = i
    nj = j
    for _ in range(4 - d):
        ni, nj = rot_ccw(ni, nj, n)
    return ni, nj


def translate_input_dir(d, face_rots):
    if face_rots == 0:
        return d
    if face_rots == 1:
        return Direction((d.value + 1) % 4)
    if face_rots == 2:
        return Direction((d.value + 2) % 4)
    assert face_rots == 3
    return Direction((d.value + 3) % 4)


def translate_dir_on_face(d, face_rots):
    if face_rots == 0:
        return d
    if face_rots == 1:
        return Direction((d.value + 3) % 4)
    if face_rots == 2:
        return Direction((d.value + 2) % 4)
    assert face_rots == 3
    return Direction((d.value + 1) % 4)


def try_move(cube, state, cube_map):
    f, i, j = state.position
    n = cube.shape[1]
    d = state.direction
    nf, ni, nj, nd = None, None, None, None
    if d == Direction.Up:
        nf, ni, nj, nd = f, i - 1, j, d
        if i == 0:
            rf = cube_map.above_of[0][f]
            nf = rf[0]
            ni, nj = translate(n - 1, j, n, rf[1])
            nd = translate_input_dir(d, rf[1])
    elif d == Direction.Right:
        nf, ni, nj, nd = f, i, j + 1, d
        if j == n - 1:
            rf = cube_map.right_of[0][f]
            nf = rf[0]
            ni, nj = translate(i, 0, n, rf[1])
            nd = translate_input_dir(d, rf[1])
    elif d == Direction.Down:
        nf, ni, nj, nd = f, i + 1, j, d
        if i == n - 1:
            rf = cube_map.below_of[0][f]
            nf = rf[0]
            ni, nj = translate(0, j, n, rf[1])
            nd = translate_input_dir(d, rf[1])
    else:
        assert d == Direction.Left
        nf, ni, nj, nd = f, i, j - 1, d
        if j == 0:
            rf = cube_map.left_of[0][f]
            nf = rf[0]
            ni, nj = translate(i, n - 1, n, rf[1])
            nd = translate_input_dir(d, rf[1])
    if cube[nf, ni, nj] != 0:
        return None
    return State((nf, ni, nj), nd)
