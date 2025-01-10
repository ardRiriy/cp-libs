import math
n = int(input())

if math.floor(n * 1.08) < 206:
    print('Yay!')
elif math.floor(n*1.08) == 206:
    print('so-so')
else:
    print(':(')