#include <bits/stdc++.h>
#include "input.hpp"

using namespace std;

// using namespace atcoder;

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
    string s; cin >> s;
    vvll dp(n+1, vll(4, inf));
    dp[0][3] = 0; // '11'からの続きとして見る
    rep(i,n) {
        ll v = s[i]-'0';
        rep2(j,1,4) {
            // cerr << "dp[" << i << "][" << j << "]= " << dp[i][j] << '\n';
            if(dp[i][j]==inf) continue;
            if(__popcount(j) == 1) {
                int nj = ((j<<1) | 1) & 3; // 1で埋めないとだめ
                chmin(dp[i+1][nj], dp[i][j] + 1-v);
            } else {
                // 1を使う
                int nj = ((j<<1) | 1) & 3;
                chmin(dp[i+1][nj], dp[i][j] + 1-v);
                if(v==0) {
                    nj = (j<<1) & 3;
                    chmin(dp[i+1][nj], dp[i][j]);
                }
            }
        }
    }
    ll ans = inf;
    rep2(i,1,4) chmin(ans, dp[n][i]);
    cout << ans << '\n';
}

int main() {
    cin.tie(0);
    ios::sync_with_stdio(false);
    int t=1;
    //cin >> t;
    while(t--)solve();
}


