#include <bits/stdc++.h>
#include "input.hpp"
#include "atcoder/lazysegtree.hpp"
using namespace std;
using namespace atcoder;

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

// 区間加算にしか興味がないので、Sの演算は何でもOK
using S = long long;
using F = long long;

const S INF = 8e18;

S op(S a, S b){ return std::min(a, b); }
S e(){ return INF; }
S mapping(F f, S x){ return f+x; }
F composition(F f, F g){ return f+g; }
F id(){ return 0; }

void solve() {
    int n; cin>> n;
    auto a = i64_vec_IN(n);
    vll indicate(n,0); iota(all(indicate), 0);
    sort(all(indicate), [&](const ll& i, const ll& j) {
        if(a[i] == a[j]) return i>j;
        else return a[i]<a[j];
    });
    lazy_segtree<S,op,e,F,mapping,composition,id> seg(n+1);
    rep(i,n) seg.set(i,0);

    set<ll> notChecked;
    rep(i,n) notChecked.insert(i);
    notChecked.insert(-1);
    notChecked.insert(n);

    for(auto i: indicate) {
        notChecked.erase(i);
        auto itr = notChecked.upper_bound(i);
        ll right = *itr;
        itr--;
        ll left = *itr;
        ll r_d = right - i;
        ll l_d = i - left;

        ll itrval = r_d + l_d - 1;
        dbg(l_d);
        dbg(r_d);
        dbg(itrval);
        seg.apply(0, min(l_d, r_d), a[i]);
        seg.apply(max(l_d, r_d), itrval+1, -a[i]);
        rep(i,n) dbg(seg.get(i));
    }

    ll cur = 0;
    vll ans(n);
    rep(i,n) {
        cur += seg.get(i);
        ans[i] = cur;
        dbg(i, cur, seg.get(i));
    }
    rep(i,n) cout << ans[i] << "\n";
}

int main() {

    cin.tie(0);
    ios::sync_with_stdio(false);
    int t=1;
    //cin >> t;
    while(t--)solve();
}


