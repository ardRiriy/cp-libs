class Dual_Fenwick_Tree:
    def __init__(self, n):
        self._n = n
        self.data = [0] * n

    # l 以上 r 未満の区間に x を加算する
    def prod(self, l, r, x):
        self._add(l, x)
        if r < self._n:
            self._add(r, -x)

    # 添え字 p の値を返す
    def get(self, p):
        return self._sum(p+1) - self._sum(0)

    def _add(self, p, x):
        p += 1
        while p <= self._n:
            self.data[p - 1] += x
            p += p & -p

    def _sum(self, r):
        s = 0
        while r > 0:
            s += self.data[r - 1]
            r -= r & -r
        return s
    
n, q = map(int, input().split())
bit = Dual_Fenwick_Tree(n+1)

for _ in range(q):
    a, b = map(int, input().split())
    a -= 1
    b -= 1
    print(bit.get(a))
    print(bit.get(b))
    if bit.get(a) % 2 != bit.get(b) % 2:
        print("No")
    else:
        print("Yes")
        bit.prod(a, b, 1)
    
