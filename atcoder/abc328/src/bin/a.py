n, x = map(int, input().split())
s = list(map(int, input().split()))
sum = 0
for si in s:
    if si <= x:
        sum += si
print(sum)
