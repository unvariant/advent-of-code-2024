shuffles = {}

"""
n n n , n n n )
"""

# first num
for i in range(3):
    # second num
    for j in range(3):
        first = (1 << (i + 1)) - 1
        second = (1 << (j + 1)) - 1
        mask = first | (second << (i + 2))
        
        shuf = [0x80] * 16
        for k in range(4):
            if k <= j:
                # vec[4 + k] = second[k]
                shuf[8 + k] = 4 + i + 2 + j - k
            if k <= i:
                # vec[k] = first[k]
                shuf[k] = 4 + i - k
        shuf[4 + 3] = 4 + i + 1 + 1 + j + 1
        shuf[0 + 3] = 4 + i + 1

        shuffles[mask] = shuf

with open("day3-digit.bin", "wb+") as fp:
    for i in range(1 << 7):
        if i in shuffles:
            fp.write(bytes(shuffles[i]))
        else:
            fp.write(b"\x80" * 16)

shuffles = {}
for i in range(3):
    for j in range(3):
        comma = i + 1
        paren = i + 1 + 1 + j + 1

        mask = (1 << comma) | (1 << paren)
        shuf = [0x80] * 16
        shuf[0] = 4 + comma
        shuf[4] = 4 + paren

        leftover = 8 - mask.bit_length()
        for k in range(1 << leftover):
            m = (k << mask.bit_length()) | mask
            shuffles[m] = shuf

with open("day3-sep.bin", "wb+") as fp:
    for i in range(1 << 8):
        if i in shuffles:
            fp.write(bytes(shuffles[i]))
        else:
            fp.write(b"\x80" * 16)