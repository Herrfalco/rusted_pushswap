#!/usr/bin/python3

import sys
import random

if __name__ == "__main__":
    if not len(sys.argv[1:]) == 1:
        exit("Error: How many number ?")

    try:
        nbrs = [str(x) for x in range(int(sys.argv[1]))]
    except ValueError:
        exit("Error: Not a number...")

    random.shuffle(nbrs)
    print(" ".join(nbrs), end="")
