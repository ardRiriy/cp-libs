import string

n, l = map(int, input().split())
vs = [input() for _ in range(n)]

prefixs = set()
for s in vs:
    for i in range(len(s)):
        prefixs.add(s[:i+1])


# # dp_{i, s, suffix} := i文字目に集合$S$が構成済みで、末尾がsuffixである物の個数 mod 998
MOD = 998244353
dp = [{} for _ in range(1<<n)]

dp[0][""] = 1

for i in range(l):
    ndp = [{} for _ in range(1<<n)] 
    for st in range(1<<n):
        for suffix, val in dp[st].items():
            for nxt in string.ascii_lowercase:
                nsf = suffix+nxt
                idx = st

                for i, si in enumerate(vs):
                    if nsf.endswith(si):
                        idx = (idx | (1 << i))
                
                key = ""
                for i in range(len(nsf)):
                    if nsf[i:] in prefixs:
                        key = nsf[i:]
                        break
                
                if key not in ndp[idx]:
                    ndp[idx][key] = 0
                ndp[idx][key] = (ndp[idx][key] + val) % MOD

    dp = ndp

ans = 0
for val in dp[(1<<n)-1].values():
    ans = (ans + val) % MOD
                    
print(ans)
                