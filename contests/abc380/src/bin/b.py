s = input()
a = []
cnt = 0

for i in range(1, len(s)):
    if s[i] == '-':
        cnt += 1
    else:
        a.append(cnt)
        cnt = 0
        
print(*a)