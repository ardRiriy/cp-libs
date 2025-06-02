a, b = map(int, input().split())
base = a//b
c = a / b

if c-base>=0.5:
    print(base+1)
else:
    print(base)