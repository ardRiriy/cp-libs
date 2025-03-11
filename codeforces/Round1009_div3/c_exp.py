n = 32

for x in range(2, n+1):
    print("======="+str(x)+"( "+bin(x)+" )=======")
    for y in range(1, x):
        xor = x^y
        if x+y > xor and xor > x-y:
            print(str(y) + ": " + bin(y))
