n, k = map(int, input().split())

x = n

def f(x: int) -> int:
    s = str(x)
    sorted_s = ''.join(sorted(s))
    reversed_s = ''.join(sorted(s, reverse=True))
    return int(reversed_s) - int(sorted_s)

for _ in range(k):
    x = f(x)
    
print(x)
