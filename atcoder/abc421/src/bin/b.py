def f(x, y):
    s = str(x+y)
    return int(s[::-1])


x, y = map(int, input().split())

for i in range(2, 10):
    res = f(x, y)
    x, y = y, res
    
print(y)