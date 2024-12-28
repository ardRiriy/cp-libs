from collections import defaultdict, deque

def count_topological_sorts_large(n, edges):
    MOD = 998244353
    
    # グラフの構築
    graph = defaultdict(list)
    in_degree = [0] * n
    for u, v in edges:
        graph[u-1].append(v-1)
        in_degree[v-1] += 1
    
    # 階乗の前計算
    fact = [1] * (n + 1)
    for i in range(1, n + 1):
        fact[i] = fact[i-1] * i % MOD
    
    # 逆元の前計算
    def pow_mod(x, n, mod):
        result = 1
        while n > 0:
            if n & 1:
                result = result * x % mod
            x = x * x % mod
            n >>= 1
        return result
    
    def inv_mod(x, mod):
        return pow_mod(x, mod-2, mod)
    
    fact_inv = [1] * (n + 1)
    for i in range(1, n + 1):
        fact_inv[i] = inv_mod(fact[i], MOD)
    
    # トポロジカルソートの順列数を計算
    result = 1
    q = deque()
    remain = n
    
    # 入次数0の頂点をキューに追加
    for i in range(n):
        if in_degree[i] == 0:
            q.append(i)
    
    # 各ステップでの選択可能な頂点数を記録
    while q:
        choices = len(q)
        if choices == 0 and remain > 0:
            return 0  # サイクルが存在する場合
        
        # 現在選べる頂点数から計算
        result = result * fact[choices] % MOD
        remain -= choices
        
        # 次のステップで選べる頂点を計算
        next_q = deque()
        visited = set()
        
        for _ in range(choices):
            v = q.popleft()
            for u in graph[v]:
                if u in visited:
                    continue
                in_degree[u] -= 1
                if in_degree[u] == 0:
                    next_q.append(u)
                    visited.add(u)
        
        q = next_q
    
    return result if remain == 0 else 0

N = 4
edges = [(2, 1), (2, 3), (1, 3), (4, 3)]

result = count_topological_sorts_large(N, edges)
print(result)  # 可能な順列の総数を出力
