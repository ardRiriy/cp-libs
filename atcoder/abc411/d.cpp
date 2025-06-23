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
    int n, q; cin >> n >> q;
    vector<string> edges;
    vector<pair<ll,ll>> g(1); // 戻り先, edge番号
    vll state(n+1, 0);

    rep(i,q) {
        int t, p; cin >> t >> p;
        if(t==1) {
            state[p] = state[0];
        } else if(t==2) {
            string s; cin >> s;
            // state[p] <- np: 
            ll np = g.size();
            ll idx = edges.size();

            g.push_back({});
            edges.push_back(s);
            
            g[np] = {state[p], idx};
            state[p] = np;
        } else {
            state[0] = state[p];
        }
    }


    vll trace;
    ll cur = state[0];
    while(cur != 0) {
        auto [ni, ei] = g[cur];
        trace.push_back(ei);
        cur = ni;
    }
    reverse(all(trace));
    string ans = "";
    for(auto i: trace) ans += edges[i];

    cout << ans << '\n';
}

int main() {
    cin.tie(0);
    ios::sync_with_stdio(false);
    int t=1;
    //cin >> t;
    while(t--)solve();
}


