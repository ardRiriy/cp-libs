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

const int inf=1ll<<30;
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

ll pc(ll val) {
    ll res = 0;

    rep(i,60) {
        if(((val>>i)&1)==1) res++;
    }

    return res;
}

int solve(){
    int n; cin >> n;
    if(n==0) return 1;
    vector<vector<bool>> has_time(n,vector<bool>(31, false));
    rep(i,n) {
        int k;
        cin >> k;
        rep(_,k) {
            int t; cin >> t;
            t--;
            has_time[i][t] = true;
        }
    }

    vll dp(n,0);
    rep(i,n) dp[i] = 1ll << i;

    rep(i,31) {
        ll nxt = 0;
        rep(j,n) {
            if(has_time[j][i]) nxt = nxt | dp[j];
        }

        if(pc(nxt) == n) {
            cout << i+1 << '\n';
            return 0;
        }

        vll ndp = dp;
        rep(j,n) if(has_time[j][i]) ndp[j] = nxt;

        dp = ndp;
        dbg(dp);

    }
    cout << "-1\n";
    return 0;
}

int main() {
    cin.tie(0);
    ios::sync_with_stdio(false);
    while(!solve());
}


