k = 103

cur = k
for i in range(100000):
    print(i+1, cur.bit_count(), cur.bit_length(), sep=" ")
    cur += k
