n = int(input())
range_v = [list(map(int, input().split())) for _ in range(n)]

ans = 0
for i in range(n):
    ans += range_v[i][1] - range_v[i][0] + 1
print(ans)
