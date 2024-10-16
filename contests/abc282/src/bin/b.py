# r, x, y = map(int, input().split()) # r, x, yのように横並びのものを受け取るとき
# a = list(map(int, input().split())) # a_0, a_1, ..., a_n を受け取るとき

n, m = map(int, input().split())
a = []

for _ in range(n):
    s = input()
    x = 0
    for i in range(m):
        if s[i] == 'o':
            x = x | (1 << i)
    a.append(x)

ans = 0
for i in range(n):
    for j in range(i+1, n):
        x = a[i] | a[j]
        if bin(x).count("1") == m:
            ans += 1

print(ans)