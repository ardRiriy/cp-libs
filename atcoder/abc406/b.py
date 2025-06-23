n, k = map(int, input().split())
a = list(map(int, input().split()))

val = 1
for ai in a:
    val *= ai
    if len(str(val)) >= k+1:
        val = 1

print(val)
