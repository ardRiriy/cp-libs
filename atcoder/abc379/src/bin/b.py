n, k = map(int, input().split())
s = input()


cnt = 0
ans = 0
for i in range(n):
    if s[i] == 'O':
        cnt += 1
        if cnt == k:
            ans += 1
            cnt = 0
    else:
        cnt = 0
        
print(ans)

