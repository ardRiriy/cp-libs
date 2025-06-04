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
    int n; cin >> n;
    string s; cin >> s;

    vector<pair<char,int>> rle;
    int cnt = 0;
    char prev = '-';
    for(char c: s) {
        if(c==prev) {
            cnt++;
        } else {
            if(prev != '-') {
                rle.push_back({prev,cnt});
            }
            prev = c;
            cnt = 1;
        }
    }
    rle.push_back({prev,cnt});


    int m = rle.size();
    vvll dp(m+1, vll(3, inf));
    dp[0][0] = 0;

    rep(i,m) {
        auto [c, w] = rle[i];
        if(c=='0') {
            chmin(dp[i+1][0], dp[i][0]);
            chmin(dp[i+1][1], dp[i][0] + w);

            chmin(dp[i+1][1], dp[i][1] + w);
            chmin(dp[i+1][2], dp[i][1]);

            chmin(dp[i+1][2], dp[i][2]);

        } else {
            chmin(dp[i+1][0], dp[i][0] + w);
            chmin(dp[i+1][1], dp[i][0]);

            chmin(dp[i+1][1], dp[i][1]);
            chmin(dp[i+1][2], dp[i][1] + w);

            chmin(dp[i+1][2], dp[i][2] + w);
        }
    }
    ll ans = inf;
    rep(i,3) {
        chmin(ans, dp[m][i]);
    }
    cout << ans << '\n';
}

int main() {
    cin.tie(0);
    ios::sync_with_stdio(false);
    int t=1;
    cin >> t;
    while(t--)solve();
}


