a = list(map(int, input().split()))
ans = 0
check = [False] * 4
for i in range(4):
    if check[i]:
        continue
    for j in range(i+1, 4):
        if check[j]:
            continue
        if a[i] == a[j]:
            ans += 1
            check[i] = True
            check[j] = True
            break
print(ans)
