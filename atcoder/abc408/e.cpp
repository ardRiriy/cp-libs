#include <bits/stdc++.h>
#include "input.hpp"
#include "atcoder/dsu.hpp"
using namespace std;
using namespace atcoder;

#ifdef ADRY
#include <dbg.h>
#else
// DO NOTHING
#define dbg(x)
#endif

#define all(v) v.begin(),v.end()
#define resort(v) sort(v.rbegin(),v.rend())
using ll = long long;
using ull = unsigned long long;
using vll=vector<ll>;
using vvll = vector<vector<ll>>;
using P = pair<ll,ll>;
using vp=vector<pair<ll, ll>>;
using djks=priority_queue<P, vp, greater<P>>;

const ll inf=1ll<<60;
#define mod10 (ll)1e9+7
#define mod99 (ll)998244353
const double PI = acos(-1);

#define rep(i,n) for (ll i=0;i<(n);++i)
#define per(i,n) for(ll i=(n)-1;i>=0;--i)
#define rep2(i,a,n) for (ll i=(a);i<(n);++i)
#define per2(i,a,n) for (ll i=(n)-1;i>=(a);--i)


template<class T>bool chmax(T &a, const T &b) { if (a<b) { a=b; return true; } return false; }
template<class T>bool chmin(T &a, const T &b) { if (b<a) { a=b; return true; } return false; }

ll dx[] = {1, 0, -1, 0, -1, 1, -1, 1};
ll dy[] = {0, 1, 0, -1, -1, 1, 1, -1};

void solve() {
    int n, m;cin >> n >> m;
    vector<tuple<int,int,ll>> e(m);
    {
        int u, v; ll w;
        rep(i,m) {
            cin >> u >> v >> w;
            u--;
            v--;
            e[i] = {u,v,w};
        }
    }


    vector<bool> deleted(m, false);
    ll ans = 0;
    per(i,31) {
        dsu uf(n);
        rep(j,m) {
            if(deleted[j]) continue;
            auto [u,v,w] = e[j];    
            if(((w>>i)&1) != 1) uf.merge(u,v);
        }
        if(uf.same(0,n-1)) {
            rep(j,m) {
                auto [u,v,w] = e[j];    
                if(((w>>i)&1) == 1) deleted[j] = true;            
            }
        } else {
            ans = ans | (1ll << i);
        }
    }
    cout << ans << '\n';
}

int main() {
    cin.tie(0);
    ios::sync_with_stdio(false);
    int t=1;
    //cin >> t;
    while(t--)solve();
}


