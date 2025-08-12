s = input()

ans = 0.0
for i in range(len(s)):
    for j in range(i+3, len(s)+1):
        if s[i] == 't' and s[j-1] == 't':
            #print(s[i:j])
            count = s[i:j].count('t')
            #print(count)
            val = (count-2) / (len(s[i:j]) - 2)
            if ans < val:
                ans = val
                
print(ans)

