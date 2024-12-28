# r, x, y = map(int, input().split()) # r, x, yのように横並びのものを受け取るとき

n = int(input())
s = list(input())

v = False

for i in range(n):
    if s[i] == "\"":
        v = not v
    if not v and s[i] == ",":
        s[i] = '.'
print(''.join(s))