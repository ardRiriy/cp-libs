di = [0, 1, 0, -1, 1, 1, -1, -1]
dj = [1, 0, -1, 0, 1, -1, 1, -1]

# r, x, y = map(int, input().split())
# a = list(map(int, input().split()))

n = int(input())
s = [input() for _ in range(n)]

p = [[0 for j in range(10)] for i in range(n)]

for i in range(n):
    for j in range(10):
        p[i][int(s[i][j])] = j

ans = 1e18

for i in range(10):
    ps = sorted([p[j][i] for j in range(n)])
    checked = [False] * n
    t = 0
    cnt = 0
    while cnt < n:
        for j in range(n):
            if not checked[j] and ps[j] == t % 10:
                checked[j] = True
                cnt += 1
                break
        if cnt < n:
            t += 1
    ans = min(ans, t)
print(ans)
