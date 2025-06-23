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

vvll gyouretuseki(vvll& a, vvll& b, ll m) {
    vvll res = {{(a[0][0]*b[0][0] + a[0][1]*b[1][0]) % m, (a[0][0]*b[0][1] + a[0][1]*b[1][1]) % m},
                {(a[1][0]*b[0][0] + a[1][1]*b[1][0]) % m, (a[1][0]*b[0][1] + a[1][1]*b[1][1]) % m}
               };

    return res;
}

void solve() {
    ll a, x, m; cin >> a >> x >> m;
    const ll N = 45;
    
    vector<vector<vector<ll>>> ruijou(45);
    ruijou[0] = {{1, 0}, {0, 1}};
    ruijou[1] = {{a, 1}, 
                 {0, 1}};
    rep2(i,2,N) {
        ruijou[i] = gyouretuseki(ruijou[i-1], ruijou[i-1], m);
    }

    vvll lhs = ruijou[0];
    rep(i,N) {
        if(i==0) continue;
        if(((x>>(i-1))&1) == 1) {
            lhs = gyouretuseki(lhs, ruijou[i], m);
        }
        // rep(i,2) rep(j,2) cerr << lhs[i][j] << " \n"[j+1==2];
    }

    cout << lhs[0][1] << '\n';
    
}

int main() {
    cin.tie(0);
    ios::sync_with_stdio(false);
    int t=1;
    //cin >> t;
    while(t--)solve();
}


