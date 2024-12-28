di = [0, 1, 0, -1, 1, 1, -1, -1]
dj = [1, 0, -1, 0, 1, -1, 1, -1]

a = input()

flag = True
for i in range(4):
    if a[i] != a[0]:
        flag = False
        
if flag:
    print("Weak")
    exit(0)

cur = int(a[0])
for i in range(3):
    if int(a[i+1]) != (cur + 1) % 10:
        print("Strong")
        exit(0)
    cur += 1
print("Weak")