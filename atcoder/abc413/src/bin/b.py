di = [0, 1, 0, -1, 1, 1, -1, -1]
dj = [1, 0, -1, 0, 1, -1, 1, -1]

n = int(input())
s = [input() for _ in range(n)]

ss = set()
for i in range(n):
    for j in  range(n):
        if i==j: continue
        t = s[i] + s[j]
        ss.add(t)
print(len(ss))