# r, x, y = map(int, input().split()) # r, x, yのように横並びのものを受け取るとき
# a = list(map(int, input().split())) # a_0, a_1, ..., a_n を受け取るとき
n = int(input())
a = [input() for _ in range(n)]


dx = [0, 1, 0, -1, 1, 1, -1, -1]
dy = [1, 0, -1, 0, 1, -1, 1, -1]

ans = 0

for i in range(n):
    for j in range(n):
        for r in range(8):
            pi = i
            pj = j
            s = 0
            for _ in range(n):
                s = s * 10 + int(a[pi][pj])
                pi = (pi+dx[r])%n
                pj = (pj+dy[r])%n
            if ans < s:
                ans = s


print(ans)
