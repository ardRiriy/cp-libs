n = 200000
q = 600000
print(n, q)
import random

for _ in range(q):
    t = random.randint(1, 3)
    if t == 1:
        u = random.randint(1, n)
        v = random.randint(1, n)
        print(t, u, v)
    else:
        u = random.randint(1, n)
        print(t, u)
    