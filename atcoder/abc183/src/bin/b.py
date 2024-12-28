x1, y1, x2, y2 = map(int, input().split())
dist = max(x1, x2) - min(x1, x2)
splt = y1 + y2
k = dist * y1 / splt
if x1 < x2:
    print(x1 + k)
else:
    print(x1 - k)
