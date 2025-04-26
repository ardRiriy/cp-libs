#include <bits/stdc++.h>
#include "atcoder/dsu"
#include "atcoder/modint"
#include "input.hpp"

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

using mint = modint998244353;

void solve() {
    int n,m;cin >> n >> m;
    dsu uf(n);
    vector<pair<int,int>>e(m);
    rep(i,m) {
        cin >> e[i].first >> e[i].second;
        e[i].first--;
        e[i].second--;
        uf.merge(e[i].first, e[i].second);
    }

    vector<int> cnt(n,0);
    for(auto[u,_]: e) cnt[uf.leader(u)]++;

    vector<bool> check(n,false);
    mint ans = 1;
    rep(i,n) {
        if(check[uf.leader(i)]) continue;
        check[uf.leader(i)]=true;
        
        if(uf.size(i)==cnt[uf.leader(i)]) ans*=2;
        else ans = 0;
    }
    cout << ans.val() << '\n';
}

int main() {
    cin.tie(0);
    ios::sync_with_stdio(false);
    int t=1;
    //cin >> t;
    while(t--)solve();
}


