di = [0, 1, 0, -1, 1, 1, -1, -1]
dj = [1, 0, -1, 0, 1, -1, 1, -1]

# r, x, y = map(int, input().split())
# a = list(map(int, input().split()))

n, r = map(int, input().split())
v = [map(int, input().split()) for _ in range(n)]

for d, a in v:
    if d == 1:
        if r >= 1600 and r <= 2799:
            r += a
    else:
        if r <= 2399 and r >= 1200:
            r += a

print(r)

