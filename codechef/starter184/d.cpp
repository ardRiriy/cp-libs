#include <bits/stdc++.h>
#include "input.hpp"
#include "atcoder/modint"
using namespace std;

using namespace atcoder;

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
    int n; cin >> n;
    auto a = i64_vec_IN(n);

    vector<int> cnt(n+1, 0);
    for(auto ai: a) cnt[ai]++;
    
    mint ans = 0, base = 2;
    int left = n - cnt[0];
    vector<vector<mint>> dp(n+2, vector<mint>(2,0));
    dp[0][0]=1;
    
    rep(i,n+1){
        if(i>1){
            ans += dp[i][1] * i * (base.pow(left));
        }
        if(i+1<n+1){
            left -= cnt[i+1];
        }

        rep(j,2) {
            dp[i+1][j] += dp[i][j] * (base.pow(cnt[i]) - 1);
        }
        dp[i+1][1] += dp[i][0];
    }

    ans += dp[n+1][1] * (n+1);
    int addr = 0;
    rep2(i,2,n+1){
        addr += cnt[i];
    }

    cout << (ans+base.pow(addr)-1).val() << '\n';

}

int main() {
    cin.tie(0);
    ios::sync_with_stdio(false);
    int t=1;
    cin >> t;
    while(t--)solve();
}


