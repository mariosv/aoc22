import sys

def main():
    data = []
    with open(sys.argv[1]) as fd:
        s = 0
        for line in fd:
            try:
                s += int(line)
            except:
                data.append(s)
                s = 0
        data.append(s)
    m = max(data)
    print(f'Result-1: {m}')
    d = sorted(data)
    print(f"Result: {sum(d[-3:])}")

if __name__ == '__main__':
    main()
