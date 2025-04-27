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

#define rep(i,n) for (ll i=0;i<(n);++i)
#define per(i,n) for(ll i=(n)-1;i>=0;--i)
#define rep2(i,a,n) for (ll i=(a);i<(n);++i)
#define per2(i,a,n) for (ll i=(n)-1;i>=(a);--i)


template<class T>bool chmax(T &a, const T &b) { if (a<b) { a=b; return true; } return false; }
template<class T>bool chmin(T &a, const T &b) { if (b<a) { a=b; return true; } return false; }

ll dx[] = {1, 0, -1, 0, -1, 1, -1, 1};
ll dy[] = {0, 1, 0, -1, -1, 1, 1, -1};

void solve() {
    int n, d; cin >> n >> d;
    auto a = i64_vec_IN(n);
    vector<int> cnt(1e6+5,0);
    for(auto ai: a) {
        cnt[ai]++;
    }

    if(d==0) {
        // 同じ要素禁止
        int ans = 0;
        for(auto c: cnt) ans += min(c, 1);
        cout << n-ans << '\n';
        return;
    }

    int ans = 0;
    rep(p, d) {
        // dで割ったあまりがpのもので、残すものを最大化
        vector<vector<int>> dp(2, vector<int>(2, 0));
        int cur = 0;
        for(int i=p; i<1e6+3; i += d) {
 
            int nxt = 1-cur;
            // 直前が残す(0)のとき
            // 選べない
            dp[nxt][1] = dp[cur][0];

            // 直前が捨てるのとき
            dp[nxt][0] = dp[cur][1] + cnt[i];
            chmax(dp[nxt][1], dp[cur][1]);

            fill(all(dp[cur]), 0);
            cur = nxt;
        }
        ans += max(dp[cur][0], dp[cur][1]);
    }
    cout << n-ans << '\n';
}

int main() {
    cin.tie(0);
    ios::sync_with_stdio(false);
    int t=1;
    //cin >> t;
    while(t--)solve();
}


