#include <bits/stdc++.h>
using namespace std;

#define all(v) v.begin(),v.end()
#define resort(v) sort(v.rbegin(),v.rend())
using ll = long long;
using ull = unsigned long long;
using vll=vector<ll>;
using vvll = vector<vector<ll>>;
using P = pair<ll,ll>;
using vp=vector<pair<ll, ll>>;

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
    int n; cin >> n;
    vll a(n), b(n), c(n);
    rep(i, n) cin >> a[i];
    rep(i, n) cin >> b[i];
    rep(i, n) cin >> c[i];

    sort(all(a));
    sort(all(b));
    sort(all(c));

    vvll dp(3, vll(n, 0));
    vvll csum(3, vll(n+1, 0));
    rep(i, n) {
        dp[0][i] = 1;
        csum[0][i+1] = csum[0][i] + dp[0][i];
    }

    // a -> b
    rep(i, n) {
        int idx = lower_bound(all(a), b[i]) - a.begin();
        ll s = csum[0][idx];
        dp[1][i] = s;
        csum[1][i+1] = csum[1][i] + dp[1][i];
    }

    rep(i, n) {
        int idx = lower_bound(all(b), c[i]) - b.begin();
        ll s = csum[1][idx];
        dp[2][i] = s;
        csum[2][i+1] = csum[2][i] + dp[2][i];
    }

    cout << csum[2][n] << '\n';
}

int main() {
    cin.tie(0);
    ios::sync_with_stdio(false);
    int t=1;
    //cin >> t;
    while(t--)solve();
}
