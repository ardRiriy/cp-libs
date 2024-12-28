n, c = map(int, input().split())
a = list(map(int, input().split()))

prev = -100000
ans = 0
for i in range(n):
    if prev + c <= a[i]:
        ans += 1
        prev = a[i]

print(ans)

