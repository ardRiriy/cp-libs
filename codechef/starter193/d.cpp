#include <bits/stdc++.h>
#include "input.hpp"

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

void dfs(int p, vector<vector<int>>& g, vll& scores, vll& depth, vll& childs) {
    ll cs = 0;
    for(auto ni: g[p]) {
        if(depth[ni] != -1) continue;
        depth[ni] = depth[p] + 1;
        dfs(ni,g,scores,depth, childs);
        cs += childs[ni];
    }

    ll res = 0;
    for(auto ni: g[p]) {
        if(depth[ni]<depth[p]) continue; //親
        ll t_s = scores[ni] + (cs - childs[ni]) * depth[p];
        chmax(res, t_s);
    }
    
    scores[p] = res+depth[p];
    childs[p] = cs+1;

    dbg(p);
    dbg(scores);
    dbg(childs);
}

void solve() {
    int n; cin >> n;
    auto g = graph_IN(n,n-1);
    ll ans = 0;
    rep(i,n) {
        vll scores(n,-1);
        vll childs(n,-1);
        vll depth(n,-1);
        depth[i] = 0;
        dfs(i,g,scores,depth,childs);
        chmax(ans, scores[i]);
    }
    cout << ans << '\n';
}

int main() {
    cin.tie(0);
    ios::sync_with_stdio(false);
    int t=1;
    
    cin >> t;
    while(t--)solve();
}


