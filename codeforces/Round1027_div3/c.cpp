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
    auto _a = i64_vec_IN(n);
    vll a;

    for(auto ai: _a) if(a.empty() || *a.rbegin() != ai) a.push_back(ai);

    int m = a.size();
    vvll dp(m+1,vll(2,-1));
    dp[0][0] = 0;
    rep(i,m) {
        // 使わない
        rep(j,2) {
            if(dp[i][j] == -1) continue;
            chmax(dp[i+1][0], dp[i][j]);
        }

        // 使う
        if(dp[i][0] != -1) chmax(dp[i+1][1], dp[i][0]+1);
        if(dp[i][1] != -1) {
            if(i==0 || a[i-1]+1 != a[i]) chmax(dp[i+1][1], dp[i][1]+1);
            else chmax(dp[i+1][1], dp[i][1]);
        }
    }
    cout << max(dp[m][0], dp[m][1]) << '\n';
}

int main() {
    cin.tie(0);
    ios::sync_with_stdio(false);
    int t=1;
    cin >> t;
    while(t--)solve();
}


