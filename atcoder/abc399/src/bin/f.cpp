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
mint ncr(int n, int r) {
    mint res = 1;
    rep(i,r) {
        res *= (n-i);
    }
    rep(i,r) {
        res /= (i+1);
    }
    return res;
}

void solve() {
    int n, k; cin >> n >> k;
    auto a = i64_vec_IN(n);
    vector<mint> csum(n+1, 0);
    rep(i,n) csum[i+1] = csum[i] + a[i];


    vector<vector<mint>> ccsum(k+1, vector<mint>(n+2, 0));
    rep(p, k+1) {
        rep(i, n+1) {
            ccsum[p][i+1] = ccsum[p][i] + csum[i].pow(p);
        }
    }

    mint ans = 0;
    rep(p,k+1){
        mint s = 0;
        rep(l,n) {
            s += (csum[l] * (-1)).pow(p) * (ccsum[k-p][n+1] - ccsum[k-p][l+1]);
        }
        dbg(s.val());
        ans += s * ncr(k,p);
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

