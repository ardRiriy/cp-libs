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
    int a, b, c; cin >> a >> b >> c;
    int sum = a + b + c;
    // dp i, j, k := 金貨i枚, 銀貨j枚, 銅貨k枚が入っている状態になる確率
    int n = 100;
    vector<vector<vector<double>>> dp(n+1, vector(n+1, vector(n+1, 0.0)));
    dp[a][b][c] = 1.0;

    rep(i, n) {
        rep(j, n) {
            rep(k, n) {
                double s = i+j+k;
                if(s==0) continue;
                dp[i+1][j][k] += dp[i][j][k] * i / s;
                dp[i][j+1][k] += dp[i][j][k] * j / s;
                dp[i][j][k+1] += dp[i][j][k] * k / s;
            }
        }
    }

    double ans = 0.0;
    rep(j,n) rep(k,n) {
        ans += dp[n][j][k] * (n + j + k - sum);
    }
    rep(i,n) rep(k,n) {
        ans += dp[i][n][k] * (n + i + k - sum);
    }
    rep(i,n) rep(j,n) {
        ans += dp[i][j][n] * (n + i + j - sum);
    }
    cout << fixed << setprecision(20) << ans << '\n';
}

int main() {
    cin.tie(0);
    ios::sync_with_stdio(false);
    int t=1;
    //cin >> t;
    while(t--)solve();
}


