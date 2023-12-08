def calc_gamma_epsilon(counters1, counters0):
    gamma = ''
    epsilon = ''
    for c1, c0 in zip(counters1, counters0):
        if c1 > c0:
            gamma += '1'
            epsilon += '0'
        else:
            gamma += '0'
            epsilon += '1'
    return int(gamma, 2), int(epsilon, 2)


def part1():
    with open('../input.txt') as f:
        data = [s.strip('\n') for s in f.readlines()]

    counters1 = [0]*len(data[0])
    counters0 = [0]*len(data[0])
    for row in data:
        for i, char in enumerate(row):
            if char == '1':
                counters1[i] += 1
            elif char == '0':
                counters0[i] += 1
    gamma, epsilon = calc_gamma_epsilon(counters1=counters1, counters0=counters0)
    print(gamma * epsilon)


def main():
    part1()


if __name__ == '__main__':
    main()

