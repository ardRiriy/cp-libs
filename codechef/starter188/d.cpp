#include <bits/stdc++.h>
#include "input.hpp"
// thanks: https://github.com/atcoder/ac-library
#include "atcoder/segtree.hpp"
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

using S = ll;
S op(S a, S b) { return a + b; }
S e() { return 0; }


void solve() {
    int n, q; cin >> n >> q;
    auto a = i64_vec_IN(n);
    vll indicate(n); iota(all(indicate), 0);
    sort(all(indicate), [&](const ll& i, const ll& j) {
        return a[i] < a[j];
    });

    vll csum(1,0);

    segtree<S,op,e> seg(n), seg2(n);
    for(auto idx: indicate) {
        ll val = seg.prod(idx, n);
        csum.push_back(*csum.rbegin() + val);
        seg.set(idx, 1);
    }
    reverse(all(indicate));

    vll csum2(1,0);
    for(auto idx: indicate) {
        ll val = seg2.prod(0,idx);
        csum2.push_back(*csum2.rbegin() + val);
        seg2.set(idx, 1);
    }
    int l, r;
    rep(_, q) {
        cin >> l >> r;
        if(l==r) cout << 0 << '\n';
        else {
            ll v1 = csum[n] - csum[l];
            ll v2 = csum2[n-r+1];
            cout << v1-v2 << '\n';
        }
    }
}

int main() {
    cin.tie(0);
    ios::sync_with_stdio(false);
    int t=1;
    cin >> t;
    while(t--)solve();
}


