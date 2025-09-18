fib = []

a, b = 1, 1

while b < int(4e12):
    if 1e6 < b:
        fib.append(b)
    a, b = b, a+b

print(len(fib))
print(fib)