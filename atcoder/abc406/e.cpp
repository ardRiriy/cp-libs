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

void solve() {
    ll n,k;
    cin>>n>>k;
    vector<vector<vector<pair<mint,mint>>>> dp(62, vector<vector<pair<mint,mint>>>(k+1, vector<pair<mint,mint>>(2, {0,0})));

    dp[61][0][0] = {1,0};

    per2(i,1,62) {
        ll p = (1ll << (i-1));
        rep(j,k+1) {
            // l=0
            if(((n>>(i-1))&1)==1) {
                // 1を選ぶ
                // 総和がいまのcnt*pだけ増える
                if(j+1<=k) {
                    dp[i-1][j+1][0].first += dp[i][j][0].first;
                    dp[i-1][j+1][0].second += dp[i][j][0].first * p + dp[i][j][0].second;
                }

                // 0を選ぶ
                dp[i-1][j][1].first += dp[i][j][0].first;
                dp[i-1][j][1].second += dp[i][j][0].second;
            } else {
                dp[i-1][j][0].first += dp[i][j][0].first;
                dp[i-1][j][0].second += dp[i][j][0].second;
            }

            // l=1
            if(j+1<=k) {
                dp[i-1][j+1][1].first += dp[i][j][1].first;
                dp[i-1][j+1][1].second += dp[i][j][1].first * p + dp[i][j][1].second;
            }

            // 0を選ぶ
            dp[i-1][j][1].first += dp[i][j][1].first;
            dp[i-1][j][1].second += dp[i][j][1].second;
        }
    }

    cout << (dp[0][k][0].second + dp[0][k][1].second).val() << '\n';
}

int main() {
    cin.tie(0);
    ios::sync_with_stdio(false);

    int t=1;
    cin >> t;
    while(t--)solve();
}


