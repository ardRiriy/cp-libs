t = int(input())

for _ in range(t):
    n = int(input())
    
    a = 10
    ans = []
    while True:
        d = a + 1
        
        if d > n:
            break
        
        if n % d == 0:
            ans.append(n//d)

        a *= 10
        
    ans.sort()
    if len(ans)==0:
        print(0)
    else:
        print(len(ans))
        print(*ans)
