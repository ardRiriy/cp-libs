di = [0, 1, 0, -1, 1, 1, -1, -1]
dj = [1, 0, -1, 0, 1, -1, 1, -1]

a, b, c, d = map(int, input().split())
ans = -1000000000000000000000000000000000000000

for i in [a, b]:
    for j in [c, d]:
        ans = max(ans, i * j)
print(ans)

