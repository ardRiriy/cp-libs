# r, x, y = map(int, input().split()) # r, x, yのように横並びのものを受け取るとき
# a = list(map(int, input().split())) # a_0, a_1, ..., a_n を受け取るとき
n = int(input())
dict = {}
ans = 0
max_val = 0
for i in range(n):
    s, x = input().split()
    x = int(x)
    if not s in dict:
        dict[s] = True
        if max_val < x:
            max_val = x
            ans = i + 1
print(ans)
