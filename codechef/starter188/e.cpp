#include <bits/stdc++.h>
#include "input.hpp"
#include "atcoder/modint.hpp"
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
using mint = modint998244353;

/* gcd(x,y) <= y-x <= x^y <= x+y, x<=y*/
void solve() {
    ll n, d; cin >> n >> d;
    vector<mint> dp(d, 0);
    dp[0] = 1;

    rep(i,n) {
        if(((d>>i)&1)==1) continue;
        ll val = (1ll<<i) % d;
        vector<mint> ndp = dp;
        rep(j,d) {
            ll nxt = (j+val)%d;
            ndp[nxt] += dp[j];
        }
        dp = ndp;
    }
    cout << (dp[0]-1).val() << '\n';
}

int main() {
    cin.tie(0);
    ios::sync_with_stdio(false);
    int t=1;

    cin >> t;
    while(t--)solve();
}


