data = open("../benches/input-4.txt").read().encode().splitlines()

def encode(b: bytes, m: list[int]):
    return ((b[0] * m[0]) + (b[1] * m[1]) + (b[2] * m[2]) + (b[3] * m[3])) & 0xff

from itertools import combinations, permutations, product
from tqdm import tqdm

ps = list(permutations("XXXXMMMMAAAASSSS\n\n\n\n", r=4))
print(f"{len(ps) = }")

for ms in list(permutations([0, 1, 2, 3, 4, 5, 6, 7, 8], r=4)):
    for m in permutations(ms):
        xmas = encode(b"XMAS", m)
        samx = encode(b"SAMX", m)
        for p in ps:
            p = "".join(p).encode()
            if encode(p, m) in [xmas, samx] and p not in [b"XMAS", b"SAMX"]:
                break
        else:
            print(m)
            print(b"XMAS", hex(encode(b"XMAS", m)))
            print(b"SAMX", hex(encode(b"SAMX", m)))