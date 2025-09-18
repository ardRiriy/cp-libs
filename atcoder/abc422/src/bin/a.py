s = input()
s1=int(s[0])
s2=int(s[2])
s2 += 1

if s2>8:
    s1 += 1
    s2 = 1
    
print(f"{s1}-{s2}")