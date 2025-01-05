n = int(input())
d = list(map(int, input().split()))
ans = 0
for i in range(n):
    for j in range(d[i]):
        month = i + 1
        day = j + 1
        flag = True
        s = str(month) + str(day)
        for k in range(len(s)-1):
            if s[k] != s[k+1]:
                flag = False
        if flag:
            ans += 1
print(ans)

