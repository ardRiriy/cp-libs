#include <bits/stdc++.h>
#include "input.hpp"
#include "atcoder/dsu"

using namespace std;

using namespace atcoder;

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
    if(n!=m) {
        cout << "No\n";
        return;
    }

    dsu uf(n);
    vector<int> cnt(n, 0);
    rep(i,m){
        int u, v; cin >> u >> v;
        u--; v--;
        uf.merge(u,v);
        cnt[u]++;
        cnt[v]++;
    }

    bool ans = true;
    rep(i,n){
        ans = ans && cnt[i]==2;
    }

    cout << ((ans && uf.size(0)==n)?"Yes\n":"No\n");
}

int main() {
    cin.tie(0);
    ios::sync_with_stdio(false);
    int t=1;
    // cin >> t;
    while(t--)solve();
}


