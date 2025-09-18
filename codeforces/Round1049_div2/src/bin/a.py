UP = int(1e8)

for x in range(1, UP+1):
    a = int(str(x) + str(2*x))
    b = 3 * x
    if a % b != 0:
        print(x)
        break