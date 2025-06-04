#include <bits/stdc++.h>
#include "input.hpp"
#include "atcoder/segtree.hpp"
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

using S = ll;
S op(S a, S b) { return max(a,b); }
S e() { return -inf; }

void solve() {
    int n, d, r; cin >> n >> d >> r;
    auto h = i64_vec_IN(n);

    segtree<S,op,e> seg(n);
    rep(i,n) seg.set(i,-inf);

    vll indicate(n); iota(all(indicate), 0);
    sort(all(indicate), [&](const ll& i, const ll& j) {
        return h[i] < h[j];
    });

    vll op_cnt(n,0);

    rep(idx,n) {
        ll i = indicate[idx];
        seg.set(i,op_cnt[i]);
        // i+dをprod
        if(idx+d<n) {
            ll j = indicate[idx+d];
            ll val = seg.prod(max(j-r, 0ll), min(j+r+1, ll(n)));
            if(val != -inf) {
                op_cnt[j] = val+1;
            }
            dbg(j);
            dbg(j-r);
            dbg(j+r+1);
        }
        // rep(i,n) cerr << op_cnt[i] << ' ';
        // cerr << '\n';

    }
    
    cout << seg.all_prod() << '\n';
}

int main() {
    cin.tie(0);
    ios::sync_with_stdio(false);
    int t=1;
    //cin >> t;
    while(t--)solve();
}


