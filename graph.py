import matplotlib.pyplot as plt


def truefalse(x: str) -> bool:
    if "false" in x:
        return False
    else:
        return True


def main():
    data_primes = []
    data_non_primes = []
    with open("parsed text.txt") as f:
        for i in f:
            temp = i.split(",")
            if truefalse(temp[2]):
                data_primes.append([int(temp[0]), float(temp[1])])
            else:
                data_non_primes.append([int(temp[0]), float(temp[1])])

    data_rotated_primes = [[row[i] for row in data_primes] for i in range(2)]
    data_rotated_non_primes = [[row[i] for row in data_non_primes] for i in range(2)]

    plt.scatter(data_rotated_primes[1], data_rotated_primes[0], label="primes", s=1)
    plt.scatter(
        data_rotated_non_primes[1], data_rotated_non_primes[0], label="non-primes", s=1
    )

    plt.xlabel("Time")
    plt.ylabel("Number")

    plt.legend()

    plt.show()


if __name__ == "__main__":
    main()
