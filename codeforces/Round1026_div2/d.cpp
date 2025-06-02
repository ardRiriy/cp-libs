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

bool j(vector<vector<pair<int, ll>>>& g, ll key, vll& a) {
    int n = g.size();
    vll max(n, -1);
    max[0] = 0;
    rep(i,n) {
        if(max[i]==-1) continue;
        max[i] += a[i];
        max[i] = min(max[i], key);
        for(auto [ni, w]: g[i]) {
            if(w > max[i]) continue;
            chmax(max[ni], max[i]);
        }
    }

    if(max[n-1] != -1) return true;
    else return false;
}

void solve() {
    int n, m; cin >> n >> m;
    auto a = i64_vec_IN(n);

    auto g = weighted_graph_IN(n,m);

    if(!j(g,inf,a)) {
        cout << "-1\n";
        return;
    }
    ll ok = 1e9+12;
    ll ng = -1;
    while(abs(ok-ng)>1) {
        ll mid = (ok+ng)>>1;
        if(j(g,mid,a)) ok = mid;
        else ng = mid;
    }
    cout << ok << '\n';
}

int main() {
    cin.tie(0);
    ios::sync_with_stdio(false);
    int t=1;
    cin >> t;
    while(t--)solve();
}


