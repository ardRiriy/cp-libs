#include <bits/stdc++.h>
#include "input.hpp"
#include "atcoder/segtree.hpp"
using namespace std;
// using namespace atcoder;

#ifdef ADRY
#include <dbg.h>
#else
// DO NOTHING
#define dbg(...)
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

using S = ll;
S op(S a, S b) { return min(a,b); }
S e() { return inf; }


void solve() {
    int n, m; cin >> n >> m;

    vector<pair<int,int>> ab(m);
    for(auto& [u,v]: ab) {
        cin >> u >> v;
    }

    vector<int> points(2*n,1<<30);
    rep(i,m) {
        auto [u,v] = ab[i];
        dbg(u,v);
        points[u-1] = i;
        points[v-1] = ~i;
    }
    dbg(points);
    vector<vector<int>> g(m+1);
    stack<int> stk;
    // for(auto [e, x, idx]: event) {
    //     if(x==1) {
    //         stk.push(idx);
    //     } else {
    //         stk.pop();
    //         //parent[idx] = ((stk.empty()) ? -1 : stk.top());
    //         g[(stk.empty()) ? 0 : stk.top()].emplace_back(idx);
    //     }
    // }
    rep(i,2*n) {
        if(i%2==1) {
            if(points[i] >= 0) {
                if(points[i]==(1<<30)) continue;
                stk.push(points[i]);
            } else {
                stk.pop();
                dbg(stk.empty()?m:stk.top());
                g[stk.empty()?m:stk.top()].emplace_back(~points[i]);
            }
        } else {
            points[i] = stk.empty() ? m : stk.top();
        }
    }

    dbg(g);

    // euler tour
    vector<int> dept(m+1,-1);
    vector<int> trace;
    vector<int> in_t(m+1,0);
    vector<int> out_t(m+1,0);
    auto euler = [&](auto self, int p, int d, int& t)->void {
        dbg(p);
        dept[p] = d;
        in_t[p] = t++;
        trace.emplace_back(p);
        for(auto ni: g[p]) {
            self(self,ni,d+1,t);
            trace.emplace_back(p);
        }
        out_t[p] = t++;
        return;
    };

    {
        int t = 0;
        euler(euler, m, 0,t);
    }
    atcoder::segtree<S,op,e> seg(trace.size());
    rep(i,trace.size()) {
        seg.set(i, dept[trace[i]]);
    }


    dbg(dept);
    int q; cin >> q;
    rep(_, q) {
        int u, v; cin >> u >> v;
        u--;
        v--;
        int lca = seg.prod(min(in_t[points[u]], in_t[points[v]]), max(out_t[points[u]], out_t[points[v]]));
        dbg(lca);
        dbg(dept[points[u]]);
        dbg(dept[points[v]]);
        cout << dept[points[u]]-lca + dept[points[v]]-lca << '\n';
    }
    
}

int main() {
    cin.tie(0);
    ios::sync_with_stdio(false);
    int t=1;
    //cin >> t;
    while(t--)solve();
}


