n = int(input())
gomi = [tuple(map(int, input().split())) for _ in range(n)]
q = int(input())

for _ in range(q):
    t, d = map(int, input().split())
    q_val, r_val = gomi[t-1]
    
    print(d + (r_val - d) % q_val)
