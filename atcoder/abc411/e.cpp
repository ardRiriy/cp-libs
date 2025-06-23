#include <bits/stdc++.h>
#include "input.hpp"
#include "atcoder/lazysegtree.hpp"
#include "atcoder/modint.hpp"
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

using mint = modint998244353;
using S = mint;
//using mint = double;
//using S = double;
//using F = double;
using F = mint;

S op(S a, S b) { return a + b; }
S e() { return 0; } 
S mapping(F f, S x) { return f * x; }
F composition(F f, F g) { return f * g; }
F id() { return 1; }


void solve() {
    int n; cin >> n;
    vvll a(n);
    vll ad(6*n);
    rep(i,n) {
        a[i] = i64_vec_IN(6);
        sort(all(a[i]));
        reverse(all(a[i]));
        rep(j,6) {
            ad[i*6+j] = a[i][j];
        }
    }

    sort(all(ad));
    ad.erase(unique(all(ad)), ad.end());
    map<ll,ll> mp;
    mp[0] = 0;
    rep(i, ad.size()) {
        mp[ad[i]] = i+1;
    }

    int m = ad.size();

    lazy_segtree<S,op,e,F,mapping,composition,id> seg(m+1);
    seg.set(0,1);

    rep(i,n) {
        map<ll,mint> addr;
        rep(j,6) {
            int idx = mp[a[i][j]];
            mint sum = seg.prod(0, idx);
            addr[idx] += sum/6;
        }

        int prev = 0;
        per(j,6) {
            int idx = mp[a[i][j]];
            seg.apply(prev,idx, S(6-j-1)/6);
            prev = idx;
        }

        for(auto [k,v]: addr) {
            mint cur = seg.get(k);
            seg.set(k, cur+v);
        }
    }

    mint ans = 0;
    for(auto val: ad) {
        ans += val * seg.get(mp[val]);
    }
    cout << ans.val() << '\n';
    //cout << ans << '\n';
}

int main() {
    cin.tie(0);
    ios::sync_with_stdio(false);
    int t=1;
    //cin >> t;
    while(t--)solve();
}


