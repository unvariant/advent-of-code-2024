masks = []
for i in range(32):
    mask = [0] * 32
    for j in range(i):
        mask[j] = 0xff
    masks.append(f"u8x32::from_array({mask})")
print(", ".join(masks))