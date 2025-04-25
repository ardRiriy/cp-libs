#include <bits/stdc++.h>
#include "input.hpp"

using namespace std;

// using namespace atcoder;

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

#define rep(i,n) for (ll i=0;i<n;++i)
#define per(i,n) for(ll i=n-1;i>=0;--i)
#define rep2(i,a,n) for (ll i=a;i<n;++i)
#define per2(i,a,n) for (ll i=n-1;i>=a;--i)


template<class T>bool chmax(T &a, const T &b) { if (a<b) { a=b; return true; } return false; }
template<class T>bool chmin(T &a, const T &b) { if (b<a) { a=b; return true; } return false; }

ll dx[] = {1, 0, -1, 0, -1, 1, -1, 1};
ll dy[] = {0, 1, 0, -1, -1, 1, 1, -1};

// 木DPする...
ll dfs1(int p, vector<vector<pair<int, ll>>>& g, vector<vp>& dists, vector<bool>& seen) {
    seen[p] = true;
    ll res = 0;
    for(auto [ni, w]: g[p]) {
        if(seen[ni]) continue;
        ll ret = dfs1(ni, g, dists, seen);
        dists[p].push_back({ni, w+ret});
        chmax(res, ret);
    }
    return res;
}

void dfs2(int p, vector<vp>& dists, ll n_w) {
    if(n_w!=-inf) {
        dists[p].push_back({1<<30, n_w});
    }

    sort(all(dists[p]), [](const P& a, const P& b) {
        return a.second > b.second;
    });

    if(dists[p][0].first!=(1<<30)) {
        int next_w=0;
        rep(i, min((int)dists[p].size(), 2)){
            next_w += dists[p][i].second;
        }
        dfs2(dists[p][0].first, dists, next_w);
    }

    rep2(i, 1, dists[p].size()) {
        if(dists[p][i].first==(1<<30)) continue;
        dfs2(dists[p][i].first, dists, dists[p][0].second + dists[p][i].second);
    }
}

void solve() {
    int n; cin >> n;
    auto g = weighted_graph_IN(n, n-1);
    vector<vp> dists(n);
    vector<bool> seen(n, false);
    dfs1(0, g, dists, seen);
    dfs2(0, dists, -inf);
    ll ans = 0;
    rep(i,n) {
        ll s = 0;
        rep(j,min((int)dists[i].size(), 2)) {
            s += dists[i][j].second;
        }
        chmax(ans, s);
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


