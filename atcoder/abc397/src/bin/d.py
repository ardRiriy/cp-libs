n = int(input())

for i in range(1, 1_000_000 + 5):
    if n % i != 0:
        continue
    target = n // i

    ok = 1
    ng = 1_000_000_000 + 20

    while ng-ok>1:
        mid = (ok+ng)>>1
        val = 3*mid*mid + 3*i*mid + i*i
  
        if val <= target:
            ok = mid
        else:
            ng = mid

    val = 3*ok*ok + 3*i*ok + i*i
    if val == target:
        x = i + ok
        print(x, ok)
        exit(0)  
print(-1)
