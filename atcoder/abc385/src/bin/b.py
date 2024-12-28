di = [0, 1, 0, -1, 1, 1, -1, -1]
dj = [1, 0, -1, 0, 1, -1, 1, -1]
dc = 'RDLU'

h, w, i, j = map(int, input().split())
s = [input() for _ in range(h)]
t = input()
seen = [[False] * w for _ in range(h)]
ans = 0

i -= 1
j -= 1

for c in t:
    for r in range(4):
        if dc[r] != c:
            continue
        ni, nj = i + di[r], j + dj[r]

        if ni < 0 or h <= ni or nj < 0 or w <= nj or s[ni][nj] == '#':
            break

        if not seen[ni][nj] and s[ni][nj] == "@":
            ans += 1
            seen[ni][nj] = True
        i, j = ni, nj
        break

print(i + 1, j + 1, ans)

