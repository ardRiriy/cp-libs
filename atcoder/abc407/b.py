x, y = map(int, input().split()) 

d = 36
n = 0

for i in range(1,7):
    for j in range(1,7):
        if i+j >= x or abs(i-j) >= y:
            n+=1


print(n/d)
