#include <bits/stdc++.h>
#include "input.hpp"
#include "atcoder/dsu.hpp"
using namespace std;
using namespace atcoder;

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
    int n; cin >> n;
    auto a = i64_vec_IN(2*n);
    for(ll& ai: a) ai--;
    dsu uf(n);

    rep(i,n) {
        uf.merge(a[2*i], a[2*i+1]);
    }

    vll edge_cnt(n);
    vll closed(n);
    vector<bool> left(n,true);

    rep(i,n) {
        edge_cnt[uf.leader(a[2*i])]++;
        if(a[2*i]==a[2*i+1] && left[a[2*i]]) {
            left[a[2*i]] = false;
            closed[uf.leader(a[2*i])]++;
        }
    }
    vector<bool> checked(n,false);
    ll ans = 0;
    rep(i,n){
        if(checked[uf.leader(i)]) continue;
        checked[uf.leader(i)] = true;

        if(edge_cnt[uf.leader(i)]>uf.size(i)-1) {
            ans += uf.size(i) - closed[uf.leader(i)];
        } else {
            ans += uf.size(i)+1;
        }
    }
    cout << ans << '\n';
}

int main() {
    cin.tie(0);
    ios::sync_with_stdio(false);
    int t=1;
    cin >> t;
    while(t--)solve();
}


