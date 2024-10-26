# r, x, y = map(int, input().split()) # r, x, yのように横並びのものを受け取るとき
# a = list(map(int, input().split())) # a_0, a_1, ..., a_n を受け取るとき

s = [list(input()) for _ in range(8)]
di = [0, 1, 0, -1, 1, 1, -1, -1]
dj = [1, 0, -1, 0, 1, -1, 1, -1]

for i in range(8):
    for j in range(8):
        if s[i][j] != '#':
            continue
        for r in range(4):
            pi = i + di[r]
            pj = j + dj[r]
            while pi < 8 and pj < 8 and pi >= 0 and pj >= 0 and s[pi][pj] != "#":
                s[pi][pj] = "@"
                pi += di[r]
                pj += dj[r]

ans = 0
for i in range(8):
    for j in range(8):
        if s[i][j] == ".":
            ans += 1


print(ans)
