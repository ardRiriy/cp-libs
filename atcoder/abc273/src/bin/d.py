h, w, i, j = map(int, input().split())
n = int(input())
set = {}
for _ in range(n):
    r, c = map(int, input().split())
    set[(r, c)] = True
q = int(input())
for _ in range(n):
    d, l = input().split()
    