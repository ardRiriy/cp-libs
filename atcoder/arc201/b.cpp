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

const int M = 60;
void solve() {
    ll n, w; cin >> n >> w;
    
    vp v(n);
    for(auto& [x,y]: v) cin >> x >> y;

    vvll g(M);
    for(auto [x, y]: v) g[x].emplace_back(y);
    ll ans = 0;
    rep(i,M) {
        bool has = ((w>>i)&1)==1;
        sort(all(g[i]));
        reverse(all(g[i]));

        if(has && !g[i].empty()) {
            ans += g[i][0];
        }

        for(int j=(has)?1:0; j<g[i].size(); j+=2) {
            dbg(g[i].size(),j);
            ll val = g[i][j];
            val += (j+1<g[i].size())?g[i][j+1]:0;
            if(i+1<M) g[i+1].emplace_back(val);
        }
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


