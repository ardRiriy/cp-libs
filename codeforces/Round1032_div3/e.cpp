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
    string l, r; cin >> l >> r;
    vll dp(4, inf);
    dp[3] = 0;

    int n = l.size();
    rep(i,n) {
        vll ndp(4,inf);
        int lc = l[i]-'0';
        int rc = r[i]-'0';

        // 0
        if(dp[0] != inf){
            // うまく選べばかならず０が達成可能
            chmin(ndp[0], dp[0]);
        }

        // 1
        if(dp[1] != inf){
            // 下限がギリ
            ll min_cost=inf;
            rep2(j,lc+1,10) {
                chmin(min_cost, (rc==j)?1ll:0ll);
            }
            if(min_cost != inf) chmin(ndp[0], dp[1]+min_cost);

            ll cost = 1;
            if(rc != lc) cost = 0;
            chmin(ndp[1], dp[1]+cost+1);
        }

        if(dp[2] != inf){
            // 下限がギリ
            ll min_cost=inf;
            rep(j,rc) {
                chmin(min_cost, (lc==j)?1ll:0ll);
            }
            if(min_cost != inf) chmin(ndp[0], dp[2]+min_cost);

            ll cost = 1;
            if(rc != lc) cost = 0;
            chmin(ndp[2], dp[2]+cost+1);
        }

        if(dp[3] != inf) {
            if(lc == rc) {
                chmin(ndp[3], dp[3]+2);
            } else {
                ll min_cost = inf;
                rep2(j,lc+1,rc) {
                    chmin(min_cost, 0ll);
                }
                if(min_cost != inf) chmin(ndp[0], dp[3]+min_cost);

                chmin(ndp[1], dp[3]+1);
                chmin(ndp[2], dp[3]+1);
            }
        }        
        dp = ndp;
        dbg(dp);
    }
    ll ans = inf;
    rep(i,4) chmin(ans,dp[i]);
    cout << ans << '\n';
}

int main() {
    cin.tie(0);
    ios::sync_with_stdio(false);
    int t=1;
    cin >> t;
    while(t--)solve();
}


