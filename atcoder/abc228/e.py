n,k,m=map(int,input().split())
P = 998244353
if m%P==0:
    # あまりは0
    exit(print(0))
    
# q: k^nをp-1で割ったあまり
q = pow(k,n,P-1)
ans = pow(m,q,P)
print(ans)
