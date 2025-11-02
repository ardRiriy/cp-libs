n, k = map(int, input().split())
s = input()

ans_len = 0
ans = set()

for i in range(n):
    t = s[i:i+k]
    if len(t) != k:
        break
    cnt = 0
    for j in range(n):
        if s[j:].startswith(t):
            cnt += 1

    if ans_len == cnt:
        ans.add(t)
    elif ans_len < cnt:
        ans_len = cnt
        ans = set()
        ans.add(t)

v = []
for s in ans:
    v.append(s)
v.sort()
print(ans_len)
print(*v)
