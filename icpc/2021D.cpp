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

//const int m = 1700;


int solve(){
    int n; cin >> n;
    if(n==0)return 1;
    auto a = i64_vec_IN(n);
    int m = 0;
    rep(i,n) m += a[i];

    vector dp(m+1, vector(m+1, -1));
    vector ep(m+1,vector(m+1,-1));

    dp[0][0] = 0;
    rep(i,n) {
        rep(j,m+1) {
            rep(k,m+1) {
                if(dp[j][k] == -1) continue;
                
                chmax(ep[j+a[i]][k], dp[j][k]);
                chmax(ep[j][k+a[i]], dp[j][k]);
                chmax(ep[j][k], int(dp[j][k]+a[i]));
            }
        }
        dp=ep;
        fill(all(ep), vector(m+1,-1));
    }

    int ans = 0;
    rep(i,m+1) rep(j,m+1) {
        chmax(ans, min(int(i),min(int(j),dp[i][j])));
    }
    cout << ans << '\n';
    return 0;
}

int main() {
    cin.tie(0);
    ios::sync_with_stdio(false);
    while(!solve());
}

