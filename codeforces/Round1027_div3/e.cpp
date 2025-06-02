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

void dfs(int p, vector<vector<int>>& g, vll& mn, vll& mx, vll& a, int parent) {
    if(p==0) {
        mn[p] = a[p];
        mx[p] = a[p];
    } else {
        mn[p] = min(-mx[parent]+a[p], a[p]);
        mx[p] = max(-mn[parent]+a[p], a[p]);
    }

    for(auto ni: g[p]) {
        if(mn[ni] == inf) {
            dfs(ni,g,mn,mx,a,p);
        }
    }
}

void solve() {
    int n; cin >> n;
    auto a = i64_vec_IN(n);
    auto g = graph_IN(n,n-1);
    vll mn(n,inf), mx(n,-inf);
    dfs(0,g,mn,mx,a,-1);
    rep(i,n) cout << mx[i] << " \n"[i+1==n];
}

int main() {
    cin.tie(0);
    ios::sync_with_stdio(false);
    int t=1;
    cin >> t;
    while(t--)solve();
}


