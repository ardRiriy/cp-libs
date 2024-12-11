import sys

n = int(input())

max = 0
max_index = 0
for i in range(1, n):
    print("? 1", i+1)
    sys.stdout.flush()
    dist = int(input())
    if max < dist:
        max = dist
        max_index = i

a = max_index
max = 0
max_index = 0
for i in range(0, n):
    if i == a:
        continue
    print("?", a+1, i+1)
    sys.stdout.flush()
    dist = int(input())
    if max < dist:
        max = dist
        max_index = i

print("!", max)
