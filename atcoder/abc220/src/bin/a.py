a, b, c = map(int, input().split()) # r, x, yのように横並びのものを受け取るとき
# a = list(map(int, input().split())) # a_0, a_1, ..., a_n を受け取るとき


for x in range(a, b+1):
    if x % c == 0:
        print(x)
        exit(0)
print(-1)