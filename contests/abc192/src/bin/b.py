from string import ascii_lowercase, ascii_uppercase

s = input()
for i in range(len(s)):
    if i%2 == 1 and s[i] in ascii_lowercase:
        print('No')
        exit(0)
    elif i%2 == 0 and s[i] in ascii_uppercase:
        print('No')
        exit(0)
print('Yes')
        