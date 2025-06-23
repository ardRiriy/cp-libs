#include <bits/stdc++.h>
#include "input.hpp"
#include "atcoder/segtree.hpp"

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

using S = ll;
S op(S a, S b) { return a + b; }
S e() { return 0; }

void dfs(int p, vector<vector<int>> &g, vector<int>& in, vector<int>& out, vector<int>& depth, vector<int>& et) {
    // 行きがけ
    in[p] = et.size();
    et.emplace_back(p);

    for(auto ni: g[p]) {
        if(depth[ni] != -1) continue;
        depth[ni] = depth[p] + 1;
        dfs(ni, g, in, out, depth, et);
        et.emplace_back(p);
    }

    out[p] = et.size();
}

void solve() {
    int n; cin >> n;
    vp edges(n-1);
    rep(i,n-1) cin >> edges[i].first >> edges[i].second, edges[i].first--, edges[i].second--;

    vector<vector<int>> g(n);
    for(auto [u,v]: edges) {
        g[u].push_back(v);
        g[v].push_back(u);
    }
    vector<int> euler_tour;
    vector<int> in(n, -1), out(n, -1), depth(n, -1);
    depth[0] = 0;

    dfs(0,g,in,out,depth,euler_tour);
    rep(i,n-1) {
        auto [u,v] = edges[i];
        if(depth[u]>depth[v]) {
            edges[i] = {v,u};
        }
    }
    
    segtree<S,op,e> segtree(euler_tour.size());
    rep(i,n) {
        segtree.set(in[i], 1ll);
    }

    ll sum = n;
    int q; cin >> q;
    int x, y;
    ll w;
    rep(_,q) {
        int t; cin >> t;
        if(t==1) {
            cin >> x >> w;
            x--;
            ll val = segtree.get(in[x]);
            segtree.set(in[x], val+w);
            sum += w;
        } else {
            cin >> y;
            y--;
            auto [u,v] = edges[y];
            ll res = segtree.prod(in[v],out[v]);
            ll other = sum-res;
            cout << abs(res-other) << '\n';
        }
    }
}

int main() {
    cin.tie(0);
    ios::sync_with_stdio(false);
    int t=1;
    //cin >> t;
    while(t--)solve();
}


