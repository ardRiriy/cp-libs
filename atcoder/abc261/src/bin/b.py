n = int(input())
a = [input() for _ in range(n)]

for i in range(n):
    for j in range(i+1, n):
        if a[i][j] == a[j][i] and a[i][j] != 'D':
            print("incorrect")
            exit(0)
        elif a[i][j] != a[j][i] and (a[i][j] == 'D' or a[j][i] == 'D'):
            print("incorrect")
            exit(0)
print("correct")
