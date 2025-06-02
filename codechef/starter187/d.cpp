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
    ll n, x; cin >> n >> x;
    auto a = i64_vec_IN(n);
    auto b = i64_vec_IN(n);

    vector<vvll> dp(n, vvll(2, vll(2, -inf)));

    rep(i,n) {
        // 新しく始める
        chmax(dp[i][0][b[i]], a[i]);

        // 取る
        rep(j, 2){
            rep(k, 2) {
                // 直前から取る
                if(i>0 && dp[i-1][j][k] != -inf) {
                    chmax(dp[i][j][k|b[i]], dp[i-1][j][k] + a[i]);
                }
                
                // 2個前から取る
                if(i>1 && dp[i-2][j][k] != -inf)  {
                    chmax(dp[i][1][k|b[i]], dp[i-2][j][k] + a[i]);
                }
            }
        }

        if(i>1) {
            rep(j,2) {
                rep(k,2) {
                    if(dp[i-2][j][k] == -inf) continue;
                    chmax(dp[i-1][j][k], dp[i-2][j][k]);
                }
            }
        }
    }

    if( dp[n-1][1][0] >= x || 
        dp[n-1][1][1] >= x || 
        dp[n-2][1][0] >= x ||
        dp[n-2][1][1] >=x     
    ) {
        cout << "0\n";
    } else {
        cout << x-max(dp[n-1][1][1], dp[n-2][1][1]) << '\n';
    }
}

int main() {
    cin.tie(0);
    ios::sync_with_stdio(false);
    int t=1;
    cin >> t;
    while(t--)solve();
}


