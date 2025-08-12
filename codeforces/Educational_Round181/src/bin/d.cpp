#include <bits/stdc++.h>
#include "input.hpp"
#include "atcoder/modint.hpp"
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

using mint = 
#ifdef ADRY
double
#else
modint998244353
#endif
;

using S = mint;
S op(S a, S b) { return a*b; }
S e() { return mint(1); }

void solve() {
    int n, m; cin >> n >> m;
    vector<vector<pair<int, mint>>> g(m+1);

    // rep(i,m) {
    //     g[i].push_back({i+1,mint(0)});
    // }

    segtree<S,op,e>seg(m);
    vector<mint> p1(m,1), p2(m,1);

    int l, r, p, q;
    rep(i,n) {
        cin >> l >> r >> p >> q;
        l--;
        g[l].push_back({r,mint(p)/q});
        seg.set(l, seg.get(l)*(mint(1)-mint(p)/q));
        if(r<m) {
            p1[r] *= (mint(1)-mint(p)/q);
        }
    }

    vector<mint> prob(m+1, 0);
    mint cur = 1;
    prob[0] = mint(1);

    rep(i,m) {
        cur /= p1[i];
        for(auto [ni, w]: g[i]) {
            //cerr << "from: " << i << ", to: " << ni << '\n';
            //cerr << seg.prod(i,ni) << " " << cur << "\n";
            prob[ni] += prob[ni]*(1-w) + w*seg.prod(i,ni)*cur/(1-w);
            //cerr << prob[ni];
        }

        cur *= seg.get(i);
    }

#ifdef ADRY
rep(i,m+1) cerr << i << "=" << prob[i] << '\n';
#else
cout << prob[m].val() << '\n';
#endif    
}

int main() {
    cin.tie(0);
    ios::sync_with_stdio(false);
    int t=1;
    //cin >> t;
    while(t--)solve();
}


