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
    ll n, t; cin >> n >> t;
    int m = n/2;
    auto a = i64_vec_IN(m);
    auto b = i64_vec_IN(n-m);
    vll as, bs;
    rep(i, 1<<m) {
        ll s = 0;
        rep(j, m) {
            if(((i>>j)&1) == 1) s += a[j];
        }
        if(s<=t) as.emplace_back(s);
    }
    rep(i, 1<<(n-m)) {
        ll s = 0;
        rep(j, n-m) {
            if(((i>>j)&1)==1) s+= b[j];
        }
        if(s<=t) bs.emplace_back(s);
    }

    ll ans = 0;
    sort(all(bs));
    for(auto ai: as) {
        ll targ = t-ai;
        // targ以下の最大値
        ll val = *(upper_bound(all(bs), targ)-1);
        dbg(ai, val);
        chmax(ans, ai + val);
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


