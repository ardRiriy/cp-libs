n = int(input())
a = [list(map(int, input().split())) for _ in range(n)]
b = [list(map(int, input().split())) for _ in range(n)]

for _ in range(4):
    flag = True
    for i in range(n):
        for j in range(n):
            if a[i][j] != b[i][j] and a[i][j] == 1:
                flag = False
    if flag:
        print("Yes")
        exit(0)

    c = [[0 for _ in range(n)] for _ in range(n)]
    for i in range(n):
        for j in range(n):
            c[i][j] = a[n-j-1][i]
    a = c

print("No")

