di = [0, 1, 0, -1, 1, 1, -1, -1]
dj = [1, 0, -1, 0, 1, -1, 1, -1]

a, b = map(int, input().split())
if a == 0:
    print("Silver")
elif b ==0:
    print("Gold")
else:
    print("Alloy")
