a = [input() for _ in range(8)]

s = "abcdefgh"
t = "87654321"

for i in range(8):
    for j in range(8):
        if a[i][j] == '*':
            print(s[j]+t[i])
