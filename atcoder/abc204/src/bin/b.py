n = input()
a = list(map(int, input().split()))

ans = 0
for ai in a:
    ans += max(ai-10, 0)
print(ans)



