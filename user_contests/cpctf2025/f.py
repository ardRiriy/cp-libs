a, b = map(int, input().split())
if a==1 or b==1:
    print("Alice")
elif a==b+1:
    print("Bob")
elif a>b:
    print("Alice")
else:
    print("Bob")