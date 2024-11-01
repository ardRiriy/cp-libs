n, x = map(int, input().split())
a = list(map(int, input().split()))

dict = {}
for i in range(n):
    dict[a[i]] = True

for i in range(n):
    if a[i] - x in dict:
        print("Yes")
        exit(0)
print("No")
