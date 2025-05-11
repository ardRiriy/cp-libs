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

const ll inf=1ll<<62;
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
    ll n, _m; cin >> n >> _m;
    auto p = i64_vec_IN(n);

    __int128_t m = _m;
    ll ok = 0;
    ll ng = m+1;
    while(abs(ok-ng)>1){
        // ll mid = (ok+ng)>>1;
        ll mid = ok + (ng - ok) / 2;

        // mid円以下の商品をすべて買う
        __int128_t usedMoney = 0;
        rep(i,n) {
            __int128_t j = (mid+p[i])/(2*p[i]);

            usedMoney += j*j*p[i];
            if(usedMoney>m) break;
        }

        if(usedMoney <= m) ok = mid;
        else ng = mid;
    }

    ll ans = 0;
    rep(i,n) {
        ll j = (ok+p[i])/(2*p[i]);
        ans += j;
        m -= j*j*p[i];
    }

    ans += m / (ok+1);
    cout << ans << '\n';
}

int main() {
    cin.tie(0);
    ios::sync_with_stdio(false);
    int t=1;
    //cin >> t;
    while(t--)solve();
}



