n = list(input())
while len(n) != 0 and n[-1] == '0':
    n.pop()

if n == n[::-1]:
    print("Yes")
else:
    print("No")