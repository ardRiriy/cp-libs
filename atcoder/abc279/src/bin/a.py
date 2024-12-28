s = input()
ans = 0
for c in s:
    if c == "v":
        ans += 1
    else:
        ans += 2

print(ans)