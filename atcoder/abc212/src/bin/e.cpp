#include <bits/stdc++.h>
#include "input.hpp"
#include "atcoder/modint.hpp"
using namespace std;
using namespace atcoder;

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

using mint = modint998244353;
void solve() {
    int n, m, k; cin >> n >> m >> k;
    auto g = graph_IN(n,m);
    rep(i,n) g[i].emplace_back(i);
    vector<mint> dp(n, mint(0));
    
    dp[0] = 1;
    mint sum = 1;

    rep(_,k-1) {    
        vector<mint> ep(n, mint(0));
        mint nsum = 0;
        rep(i,n) {
            mint si = 0;
            for(auto ni: g[i]) {
                si += dp[ni];
            }
            ep[i] = sum - si;
            nsum += ep[i];
        }
        sum = nsum;
        dp = ep;

    }

    for(auto ni: g[0]) {
        sum -= dp[ni];
    }
    cout << sum.val() << '\n';
}

int main() {
    cin.tie(0);
    ios::sync_with_stdio(false);
    int t=1;
    //cin >> t;
    while(t--)solve();
}
