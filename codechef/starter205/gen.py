import random

t = 100000

print(t)
for _ in range(t):
    n = random.randint(1, 7)
    f = [0] * n
    for _ in range(n):
        ai = random.randint(1, n)
        f[ai-1] += 1

    print(n)
    f[0] += n
    print(*f)