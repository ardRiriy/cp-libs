s = input()

i = 0
ans = 0

while i < len(s):
    if i != len(s)-1 and s[i] == '0' and s[i+1] == '0':
        i += 1
    ans += 1
    i += 1
print(ans)
