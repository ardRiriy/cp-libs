a, b, k = map(int, input().split())

n = a
ans = 0

while n < b:
    n = n * k
    ans += 1
    
print(ans)

