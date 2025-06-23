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

mint ncr(vector<mint>& f, int n, int r) {
    assert(n>=r);
    return f[n] / f[r] / f[n-r];
}

void solve() {
    int a, b, c, d; cin >> a >> b >> c >> d;
    ll sum = a + b + c + d;
    vector<mint> factor(sum+1, 1);
    rep(i,sum) factor[i+1] = factor[i] * (i+1);

    mint ans = 0;

    rep(i,sum) {
        if(a>i+1) continue;
        if(c>sum-i-1) continue;
        if(i+1>a+b) continue;
        ans += ncr(factor ,i,a-1) * ncr(factor, sum-i-1, c);
    }

    cout << ans.val() << '\n';
}

int main() {
    cin.tie(0);
    ios::sync_with_stdio(false);
    int t=1;
    //cin >> t;
    while(t--)solve();
}


