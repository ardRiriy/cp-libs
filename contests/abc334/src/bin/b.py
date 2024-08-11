a, m, l, r = map(int, input().split())

l = l - a
r = r - a

ld = l // m
rd = r // m

if l % m == 0:
    ld -= 1

print(rd - ld)
