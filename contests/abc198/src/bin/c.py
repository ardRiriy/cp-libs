import math
r, x, y = map(int, input().split())
dist = math.hypot(x, y)
if r == dist:
    print(1)
elif dist <= r * 2:
    print(2)
else:
    ans = math.ceil(dist / r)
    print(ans)