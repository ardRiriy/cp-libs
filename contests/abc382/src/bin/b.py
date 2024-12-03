n, d = map(int, input().split())
s = list(input())

cnt = 0
for i in range(n-1, -1, -1):
    if s[i] == '@':
        cnt += 1
        s[i] = '.'
        if cnt == d:
            break

print(''.join(s))