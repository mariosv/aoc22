from enum import Enum

class Direction(Enum):
    Right = 0
    Down = 1
    Left = 2
    Up = 3

    def __str__(self):
        return ['>', 'v', '<', '^'][self.value]


def rotate_cw(d: Direction):
    return Direction((d.value + 1) % 4)


def rotate_ccw(d: Direction):
    if d == Direction.Up:
        return Direction.Left
    if d == Direction.Left:
        return Direction.Down
    if d == Direction.Down:
        return Direction.Right
    assert d == Direction.Right
    return Direction.Up

    #return Direction((d.value + 3) % 4)


