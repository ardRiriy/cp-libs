l, r, x, y = map(int, input().split())

ir = False
ib = False

ans = 0

for i in range(0, 101):
    if i == l:
        ir = True
    elif i == r:
        ir = False

    if i == x:
        ib = True
    elif i == y:
        ib = False
        
    if ir and ib:
        ans += 1

print(ans)
    