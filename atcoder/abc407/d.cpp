#include <bits/stdc++.h>
#include "input.hpp"

using namespace std;
// using namespace atcoder;

#ifdef ADRY
#include <dbg.h>
#else
// DO NOTHING
#define dbg(...)
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

int h, w;
ll dfs(int i, int j, vvll& a, vector<vector<bool>>& state) {
    if(i==h) {
        // 終了、SCORE計算
        ll res = 0;
        rep(x, h) {
            rep(y, w) {
                if(!state[x][y]) res ^= a[x][y];
            }
        }
        return res;
    }

    int ni = (j+1==w)?i+1:i;
    int nj = (j+1==w)?0:j+1;
    ll res = 0;
    // 置かない
    chmax(res, dfs(ni,nj,a,state));
    if (!state[i][j]) {
        // 下向き
        if(i+1<h && !state[i+1][j]) {
            state[i][j] = true;
            state[i+1][j] = true;
            chmax(res, dfs(ni,nj,a,state));
            state[i][j] = false;
            state[i+1][j] = false;
        } 
        // 左
        if(j+1<w && !state[i][j+1]) {
            state[i][j] = true;
            state[i][j+1] = true;
            chmax(res, dfs(ni,nj,a,state));
            state[i][j] = false;
            state[i][j+1] = false;
        }
    }
    return res;
}

void solve() {
    cin >> h >> w;
    vvll a(h, vll(w)); rep(i,h) rep(j,w) cin >> a[i][j];
    vector<vector<bool>> state(h, vector<bool>(w, false));
    cout << dfs(0,0,a,state) << '\n';
}

int main() {
    cin.tie(0);
    ios::sync_with_stdio(false);
    int t=1;
    //cin >> t;
    while(t--)solve();
}


