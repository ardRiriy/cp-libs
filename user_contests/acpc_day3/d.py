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

n = int(input())
a = list(map(int, input().split()))


ok = 1_000_000_000_000_000_001
ng = -1

while abs(ok-ng)>1:
    mid = (ok+ng)>>1
    sum = 0
    for i in range(8):
        sum += a[i]*comb(mid, i+1)

    #print("sum= ", sum, sep=" ")
    if sum >= n:
        ok = mid
    else:
        ng = mid
# for i in range(3):
#     print("a[i]*comb_sum(i+1, 4)= ", a[i]*comb_sum(i+1, 4))


print(ok)