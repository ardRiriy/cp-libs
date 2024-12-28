# r, x, y = map(int, input().split()) # r, x, yのように横並びのものを受け取るとき
# a = list(map(int, input().split())) # a_0, a_1, ..., a_n を受け取るとき
n, w = map(int, input().split())
a = list(map(int, input().split()))
ans = [False] * (w+1)

for i in range(n):
    if a[i] > w:
        continue
    ans[a[i]] = True
    for j in range(i+1, n):
        s = a[i] + a[j]
        if s > w:
            continue
        ans[s] = True
        for k in range(j+1, n):
            if s + a[k] <= w:
                ans[s + a[k]] = True

print(ans.count(True))