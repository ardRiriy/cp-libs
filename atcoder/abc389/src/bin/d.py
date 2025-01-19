r = int(input())
r2 = r**2
ans = 0

import math
for i in range(1, r):
    p = math.floor(math.sqrt(r2 - pow(i+0.5, 2)) - 0.5)
    val = p * 2 + 1
    ans += val 

ans *= 2
ans += math.floor(math.sqrt(r2 - (0.5**2)) - 0.5)*2 + 1

print(ans)