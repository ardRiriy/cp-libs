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
    auto a = i64_vec_IN(n);
    vector<tuple<int,int,int>> queries(q);
    int x,y,z;
    rep(i,q) {
        cin >> x >> y >> z;
        x--;y--;z--;
        queries[i] = {x, y, z};
    }

    vll least = a;
    per(i,q) {
        auto [x,y,z] = queries[i];
        chmax(least[x], least[z]);
        chmax(least[y], least[z]);
        least[z] = 1;

        dbg(least);
    }
    vll ans = least;
    
    rep(i,q) {
        auto [x,y,z] = queries[i];
        least[z] = min(least[x], least[y]);
    }
    if(a==least) rep(i,n) cout << ans[i] << " \n"[i+1==n];
    else cout << "-1\n";

}

int main() {
    cin.tie(0);
    ios::sync_with_stdio(false);
    int t=1;
    cin >> t;
    while(t--)solve();
}


