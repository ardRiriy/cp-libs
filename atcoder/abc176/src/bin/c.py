n = int(input())
a = list(map(int, input().split()))
ans = 0
mx_a = 0
for i in range(n):
    if mx_a >= a[i]:
        ans += mx_a - a[i] 
    else:
        mx_a = a[i]
print(ans)
