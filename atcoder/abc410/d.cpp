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

void solve() {
    int n, m; cin >> n >> m;
    vector<vp> g(n);
    int u, v, w;
    rep(i,m) {
        cin >> u>> v >> w;
        u--;
        v--;
        dbg(u, v, w);
        g[u].push_back({v,w});
    }
    dbg("====");

    vector<vector<bool>> seen(n, vector(1<<11, false));
    seen[0][0] = true;
    queue<P> que;
    que.push({0,0});
    while(!que.empty()) {
        auto [p, ws] = que.front();
        dbg(p, ws);
        que.pop();

        for(auto [ni, w]: g[p]) {
            auto cost = ws ^ w;
            if(seen[ni][cost]) continue;
            seen[ni][cost] = true;
            que.push({ni, cost});
        }
    }
    dbg("===");
    ll ans = inf;
    rep(i,1<<11) {
        if(seen[n-1][i])chmin(ans,i);
    }
    cout << ((ans==inf)?-1:ans) << '\n';
}

int main() {
    cin.tie(0);
    ios::sync_with_stdio(false);
    int t=1;
    //cin >> t;
    while(t--)solve();
}


