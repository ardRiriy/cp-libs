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
    ll n, k; cin >> n >> k;
    

    vvll dp(40, vll(4, 0));
    dp[39][0] = 1;
    bool found = false;
    per2(i, 1, 40) {
        found = found || (n >> (i-1)) & 1 == 1;

        // dp[i][0]
        if(k > i-1) { // 商品iが存在する
            if((n >> (i-1)) & 1 == 1) {
                dp[i-1][1] += dp[i][0];
                dp[i-1][2] += dp[i][0];
            } else {
                dp[i-1][0] += dp[i][0];
            }
        } else {
            if(found) dp[i-1][2] += dp[i][0];
            else dp[i-1][0] += dp[i][0];
        }

        // dp[i][1]
        if(k > i-1) {
            if((n >> (i-1)) & 1 == 1) {
                dp[i-1][1] += dp[i][1];
                dp[i-1][2] += dp[i][1];
            } else {
                dp[i-1][0] += dp[i][1];
            }
        } else {
            dp[i-1][1] += dp[i][1];
        }

        if(k > i-1) {
            rep2(j, 2, 4) {
                rep2(k, 2, 4) {
                    dp[i-1][k] += dp[i][j];
                }
            }
        } else {
            rep2(j, 2, 4) {
                dp[i-1][j] += dp[i][j];
            }
        }
    }
    ll ans = 0;
    rep(i, 4) {
        ans += dp[0][i];
    }
    cout << ans << '\n';
}

int main() {
    cin.tie(0);
    ios::sync_with_stdio(false);
    int t;
    cin >> t;
    while(t--)solve();
}