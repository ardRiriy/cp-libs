t = int(input())
for _ in range(t):
    n, k = map(int, input().split())
    x = 1
    for _ in range(k):
        if x>n+1:
            break
        x *= 2
    print(min(n+1, x))