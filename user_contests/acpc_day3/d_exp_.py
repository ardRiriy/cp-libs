def comb(n, k):
    if n<k:
        return 0
    
    numer = 1
    for i in range(k):
        numer *= (n-i)
    
    denom = 1
    for i in range(k):
        denom *= i+1
    return numer // denom


n=4
sum = 0
for t in range(5000000):
    sum += comb(t, n-1)

print("t=", t, " / sum = ", sum)

