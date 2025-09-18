x, c = map(int, input().split())

ans = 0
for i in range(1, x+1):
    if i * 1000 > x:
        break
    money = i * 1000 + i * c
    if money <= x:
        ans = i * 1000
print(ans)
        
