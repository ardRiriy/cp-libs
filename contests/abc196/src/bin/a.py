a, b = map(int, input().split())
c, d = map(int, input().split())

ans = -1e6
for x in range(a, b+1):
    for y in range(c, d+1):
        if ans < x - y:
            ans = x - y

print(ans)
