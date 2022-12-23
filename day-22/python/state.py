from direction import rotate_cw, rotate_ccw


class State(object):
    def __init__(self, position, direction):
        self.position = position
        self.direction = direction

    def __repr__(self):
        return f"{str(self.position)}{str(self.direction)}"


def rotate(state, d):
    f = rotate_cw
    if d == 'L':
        f = rotate_ccw
    return State(state.position, f(state.direction))
