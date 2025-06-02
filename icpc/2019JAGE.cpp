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


int solve(){
    int n, m;cin >> n >> m;
    if(n==0)return 1;

    vvll a(n, vll(m));
    rep(i,n) rep(j,m) cin >> a[i][j];
    
    // vvll g(m);
    vll g(m, 0);
    rep(x,m) {
        g[x] = g[x] | (1ll << x);
        rep2(y,x+1,m) {
            vll indicate(n); iota(all(indicate), 0);
            sort(all(indicate), [&](const ll& i, const ll& j) {
                if(a[i][x] == a[j][x]) return a[i][y] < a[j][y];
                else return a[i][x] < a[j][x];
            });

            bool flag = true;
            rep(i,n-1) {
                flag = flag && a[indicate[i]][y] <= a[indicate[i+1]][y];
            }
            if(flag) {
                g[x] = g[x] | (1ll << y);
                g[y] = g[y] | (1ll << x);
            }
        }
    }

    ll l_size = m/2;
    ll r_size = m-l_size;

    vector<bool> is_clique_l(1<<l_size, false);
    vector<bool> is_clique_r(1<<r_size, false);

    rep2(i, 1, (1<<l_size)) {
        bool flag = true;
        rep(j, l_size) {
            if(((1>>j)&1) != 1) continue; // 対象の辺じゃない場合はスキップ
            flag = flag && ((g[j] & i) == i);
        }
        is_clique_l[i] = flag;
    }

    rep2(i, 1, (1<<r_size)) {
        ll k = (i<<l_size);
        bool flag = true;
        rep(j, r_size) {
            if(((i>>j)&1) != 1) continue;
            flag = flag && ((g[l_size+j] & k) == k);
        }
        is_clique_r[i] = flag;
    }

    

}

int main() {
    cin.tie(0);
    ios::sync_with_stdio(false);
    while(!solve());
}


