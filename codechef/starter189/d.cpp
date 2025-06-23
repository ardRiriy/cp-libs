#include <bits/stdc++.h>
#include "input.hpp"

using namespace std;
// using namespace atcoder;

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
    int n, k; cin >> n >> k;
    auto a = i64_vec_IN(n);
    vvll g(k+1);

    rep(i,n) {
        g[a[i]].emplace_back(i);
    }

    vll dist(n, inf);
    for(auto i: g[k]) {
        dist[i] = 0;
    }

    per2(t, 1, k) {
        for(auto i: g[t]) {
            auto itr = lower_bound(all(g[t+1]), i);
            if(itr != g[t+1].end()) {
                auto val = *itr;
                chmin(dist[i], dist[val] + abs(val-i));
            }
            if(itr != g[t+1].begin()) {
                itr--;
                auto val = *itr;
                chmin(dist[i], dist[val] + abs(val-i));
            }
        }
    }

    vll ans(n);
    rep(i,n) {
        if(a[i] == 1) ans[i] = dist[i];
        else {
            auto itr = upper_bound(all(g[1]), i);
            ll res = inf;
            if(itr != g[1].end()) {
                auto val = *itr;
                chmin(res, dist[val] + abs(val-i));
            }
            if(itr != g[1].begin()) {
                itr--;
                auto val = *itr;
                chmin(res, dist[val] + abs(val-i));
            }
            ans[i] = res;
        }
    }

    rep(i,n) cout << ans[i] << " \n"[i+1==n];
}

int main() {
    cin.tie(0);
    ios::sync_with_stdio(false);
    int t=1;
    cin >> t;
    while(t--)solve();
}


