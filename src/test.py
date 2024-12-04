data = open("../benches/input-3.txt").read()

for line in open("test.txt").read().splitlines():
    a, b = map(int, line.split())
    if a == 0 and b == 0:
        continue
    m = f"mul({a},{b})"
    assert m in data, m