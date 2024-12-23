n = int(input())
s = input()

for c in s:
    if not c in "TeraCoder":
        print("No")
        exit(0)
print("Yes")