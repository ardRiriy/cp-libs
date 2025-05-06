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
    int n, m; cin >> n >> m;
    auto c = i64_vec_IN(n);

    vector<vector<bool>> a(m, vector<bool>(n,false));
    rep(i,m) {
        int k; cin>> k;
        rep(_,k){
            int p; cin >> p;
            p--;
            a[i][p] = true;
        }
    }

    vvll dp(1<<n*2, vll(m, 0)); // ある状態iにおいて動物jを見た回数

    ll ans = inf;
    rep(i, 1<<(n*2)){
        bool flag = true;
        rep(j,m){
            flag = flag && dp[i][j] >= 2;
        }
        
        if(flag) {
            ll cnt = 0;
            rep(j,2*n){
                if(((i>>j)&1)==1) {
                    cnt += c[j%n];
                }
            }
            chmin(ans, cnt);
            continue;
        }

        
        // 最上位bitを見つける
        ll start = inf;
        per(j,2*n) {
            if(((i>>j)&1)==1) {
                start = j+1;
                break;
            }
        }
        if(start==inf) start = 0;

        rep2(j, start, 2*n) {
            int nxt = i | (1 << j);
            if(nxt==i) continue;

            // j % nを訪問
            rep(k,m){
                dp[nxt][k] = dp[i][k] + ((a[k][j%n])?1:0);
            }
        }
    }
    cout << ans << '\n';
}

int main() {
    cin.tie(0);
    ios::sync_with_stdio(false);
    int t=1;
    //cin >> t;
    while(t--)solve();
}


