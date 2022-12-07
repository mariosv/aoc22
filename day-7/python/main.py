import sys


class File(object):
    def __init__(self, name, size):
        self.name = name
        self.size = size

class Dir(object):
    def __init__(self, name, parent):
        self.name = name
        self.parent = parent
        self.dirs = []
        self.files = []
        self._size = None

    def filesize(self):
        if self._size is None:
            s = 0
            for f in self.files:
                s += f.size
            for d in self.dirs:
                s += d.filesize()
            self._size = s
        return self._size


def parse_change_dir(s, cwd, root):
    if s[2] == '..':
        assert cwd is not None
        cwd.parent.dirs.append(cwd)
        cwd = cwd.parent
    else:
        nd = Dir(s[2], cwd)
        if cwd is None:
            assert s[2] == '/'
            root = nd
        cwd = nd
    return cwd, root


def parse_files(fd, cwd):
    while True:
        line = fd.readline()
        if not line:
            cwd.parent.dirs.append(cwd)
            break
        if line.startswith('$'):
            break
        s = line.split()
        if s[0] == 'dir':
            pass
        else:
            cwd.files.append(File(s[1], int(s[0])))
    return cwd, line


def parse_input(filename):
    with open(filename) as fd:
        cwd = None
        line = fd.readline()
        root = None
        while line:
            assert(line.startswith('$'))
            s = line.split()
            if s[1] == 'cd':
                cwd, root = parse_change_dir(s, cwd, root)
                line = fd.readline()
            else:
                assert s[1] == 'ls'
                cwd, line = parse_files(fd, cwd)
    return root


def problem_1(root):
    s = 0
    st = [root]
    while True:
        try:
            d = st.pop()
        except:
            break
        if d.filesize() < 100000:
            s += d.filesize()
        for t in d.dirs:
            st.append(t)
    return s


def problem_2(root):
    total = 70000000
    needed = 30000000
    tofree = needed - (total - root.filesize())
    s = sys.maxsize
    st = [root]
    while True:
        try:
            d = st.pop()
        except:
            break
        if d.filesize() > tofree:
            s = min(s, d.filesize())
        for t in d.dirs:
            st.append(t)
    return s


def parse_args():
    if len(sys.argv) != 2:
        sys.exit('I need a file and just a file')
    return sys.argv[1]


def main():
    filename = parse_args()
    root = parse_input(filename)
    print('Day 7, problem 1: ', problem_1(root))
    print('Day 7, problem 2: ', problem_2(root))


if __name__ == '__main__':
    main()
