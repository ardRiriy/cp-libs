t = int(input())

for _ in range(t):
    a, b = map(int, input().split())
    
    if a % 2 == 0:
        if b % 2 == 1:
            print("-1")
            continue
        k = b//2
        print(a * k + 2)
    else:
        if b % 2 == 0:
            if b % 4 != 0:
                print("-1")
                continue
            k = b//2
            print(a*k+2)
        else:
            print(a*b + 1)

