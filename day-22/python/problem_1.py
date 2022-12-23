from direction import Direction
import state
from state import (State, rotate)


def solve(maze, moves):
    state = find_start(maze)
    states = []
    states.append(state)
    for m in moves:
        if m.rotate is not None:
            state = rotate(state, m.rotate)
            states.append(state)
        else:
            assert m.steps
            for _ in range(m.steps):
                nstate = try_move(maze, state)
                if nstate is None:
                    break
                state = nstate
                states.append(state)
    final = states[-1].position
    final_dir = states[-1].direction
    return 1000 * (final[0] + 1) + 4 * (final[1] + 1) + final_dir.value


def find_start(maze):
    return State((0, maze[0].index('.')), Direction.Right)


def try_move(maze, state):
    i, j = state.position
    d = state.direction
    ni, nj = None, None
    if d == Direction.Up:
        ni, nj = i - 1, j
        if i == 0 or len(maze[ni]) <= j or maze[ni][j] == ' ':
            ni = len(maze) - 1
            while j >= len(maze[ni]) or maze[ni][j] == ' ':
                ni -= 1
    elif d == Direction.Right:
        ni, nj = i, j + 1
        if j == (len(maze[i]) - 1):
            nj = 0
            while maze[i][nj] == ' ':
                nj += 1
    elif d == Direction.Down:
        ni, nj = i + 1, j
        if i == (len(maze) - 1) or len(maze[ni]) <= j or maze[ni][j] == ' ':
            ni = 0
            while j >= len(maze[ni]) or maze[ni][j] == ' ':
                ni += 1
    else:
        assert d == Direction.Left
        ni, nj = i, j - 1
        if j == 0 or maze[i][nj] == ' ':
            nj = len(maze[i]) - 1
            while maze[i][nj] == ' ':
                nj -= 1
    if maze[ni][nj] != '.':
        return None
    return State((ni, nj), state.direction)
