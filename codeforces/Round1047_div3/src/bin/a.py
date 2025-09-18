t = int(input())

for _ in range(t):
    k, x = map(int, input().split())
    print(x * pow(2,k))