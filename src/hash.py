data = open("../benches/input-3.txt").read().encode()

def encode(b: bytes):
    return b[0] * 2 + b[1] + b[2] + b[3]

for i in range(len(data) - 3):
    part = data[i:i+4]
    h = encode(part)
    if h == encode(b"mul(") and h != b"mul(":
        print(part)