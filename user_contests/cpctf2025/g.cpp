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

void solve() {
    int n, m, k; cin >> n >> m >> k;
    vll c(m, -1); rep(i,m) cin >> c[i];
    vector<vp> g(n);
    rep(i,m) {
        int u, v; cin >> u >> v;
        u--;v--;
        g[u].push_back({v, c[i]});
        g[v].push_back({u, c[i]});
    }

    vll dist(n*(k+1), inf);
    djks pq;
    pq.push({0,n*k});
    while(!pq.empty()) {
        auto [c, p] = pq.top(); pq.pop();
        if(dist[p]!=inf) continue;

        dist[p] = c;
        int ck = p/n;
        for(auto[ni, w]: g[p%n]) {
            // クーポンなし
            {
                int next = n*ck+ni;
                if(dist[next]==inf) {
                    pq.push({c+w, next});
                }
            }
            {
                // クーポンあり
                if(ck>0) {
                    int next = n*(ck-1)+ni;
                    if(dist[next]==inf) {
                        pq.push({c, next});
                    }
                }
            }
        }
    }
    ll ans = inf;
    rep(i,k+1) chmin(ans, dist[n*(i+1)-1]);
    cout << ((ans==inf)?-1:ans) << '\n';
}

int main() {
    cin.tie(0);
    ios::sync_with_stdio(false);
    int t=1;
    //cin >> t;
    while(t--)solve();
}


