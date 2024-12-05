data = open("../benches/input-4.txt").read().encode().splitlines()

def encode4(b: bytes, m: list[int]):
    return ((b[0] * m[0]) + (b[1] * m[1]) + (b[2] * m[2]) + (b[3] * m[3])) & 0xff

def encode3(b: bytes, m: list[int]):
    return ((b[0] * m[0]) + (b[1] * m[1]) + (b[2] * m[2])) & 0xff

from itertools import combinations, permutations, product
from tqdm import tqdm
import sys

mode = int(sys.argv[1])

if mode == 0:
    ps = list(permutations("XXXXMMMMAAAASSSS\n\n\n\n", r=4))
    print(f"{len(ps) = }")

    # search for a good hashing function
    # must handle newlines properly

    for ms in list(permutations([0, -1, 1, -2, 2, -3, 3, -4, 4], r=4)):
        for m in permutations(ms):
            xmas = encode4(b"XMAS", m)
            samx = encode4(b"SAMX", m)
            for p in ps:
                p = "".join(p).encode()
                if encode4(p, m) in [xmas, samx] and p not in [b"XMAS", b"SAMX"]:
                    break
            else:
                print(m)
                print(b"XMAS", hex(xmas))
                print(b"SAMX", hex(samx))
elif mode == 1:
    ps = list(permutations("XXXXMMMMAAAASSSS\n\n\n\n", r=3))
    print(f"{len(ps) = }")

    for ms in list(permutations([0, -1, 1, -2, 2, -3, 3, -4, 4], r=3)):
        for m in permutations(ms):
            mas = encode3(b"MAS", m)
            sam = encode3(b"SAM", m)
            for p in ps:
                p = "".join(p).encode()
                if encode3(p, m) in [mas, sam] and p not in [b"MAS", b"SAM"]:
                    break
            else:
                print(m)
                print(b"MAS", hex(mas))
                print(b"SAM", hex(sam))