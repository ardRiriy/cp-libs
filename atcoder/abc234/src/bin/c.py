n = int(input())
res = format(n, 'b')
ans = []
for c in res:
    if c == '0':
        ans.append(c)
    else:
        ans.append('2')

print(''.join(ans))
