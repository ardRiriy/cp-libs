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
    auto g = graph_IN(n,m);
    // vll dist(1<<n, inf);
    vvll dist(1<<n, vll(n, inf));
    dist[0][0]=0;
    queue<P>que; que.push({0,inf});

    while(!que.empty()){
        auto [state, last_v] = que.front(); que.pop();
        if(last_v==inf) {
            // 任意の頂点に移動可能
            rep(i,n){
                ll nxt = state ^ (1<<i);
                dist[nxt][i] = dist[state][0] + 1;
                que.push({nxt, i});
            }
        } else {
            for(auto ni: g[last_v]) {
                ll nxt = state ^ (1<<ni);
                if(dist[nxt][ni] != inf) continue;
                dist[nxt][ni] = dist[state][last_v] + 1;
                que.push({nxt, ni});
            }
        }
    }

    ll sum = 0;
    
    for(auto& v: dist) {
        ll mn = inf;
        rep(i,n) {
            chmin(mn, v[i]);
        }
        sum += mn;
    }
    cout << sum << '\n';
}

int main() {
    cin.tie(0);
    ios::sync_with_stdio(false);
    int t=1;
    //cin >> t;
    while(t--)solve();
}


