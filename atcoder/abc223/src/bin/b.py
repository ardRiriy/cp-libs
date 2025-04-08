s=list(input())
mis=s.copy()
mas=s.copy()

for _ in range(len(s)):
    c=s[0]
    s.remove(c)
    s.append(c)
    if mis > s:
        mis = s.copy()
    if mas < s:
        mas = s.copy()

print(''.join(mis))
print(''.join(mas))



