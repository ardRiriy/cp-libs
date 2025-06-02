#include <bits/stdc++.h>
#include "input.hpp"
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
S op(S a, S b) { return min(a,b); }
S e() { return inf; }
using T = ll;
T t_op(T a, T b) { return max(a,b); }
T t_e() { return -inf; }

void solve() {
    int n; cin >> n;
    segtree<S,op,e> x_min_seg(n), y_min_seg(n);
    segtree<T,t_op,t_e> x_max_seg(n), y_max_seg(n);
    ll x, y;
    rep(i,n) {
        cin >> x >> y;
        x_max_seg.set(i,x);
        x_min_seg.set(i,x);
        y_max_seg.set(i,y);
        y_min_seg.set(i,y);
    }
    if(n==1) {
        cout << "1\n";
        return;
    }

    ll ans = inf;
    rep(i,n) {
        ll min_x = inf, min_y = inf;
        if(i>0) {
            chmin(min_x, x_min_seg.prod(0,i));
            chmin(min_y, y_min_seg.prod(0,i));
        }
        if(i+1<n) {
            chmin(min_x, x_min_seg.prod(i+1,n));
            chmin(min_y, y_min_seg.prod(i+1, n));
        }
        // max
        ll max_x = -inf, max_y = -inf;
        if(i>0) {
            chmax(max_x, x_max_seg.prod(0,i));
            chmax(max_y, y_max_seg.prod(0,i));
        }
        if(i+1<n) {
            chmax(max_x, x_max_seg.prod(i+1,n));
            chmax(max_y, y_max_seg.prod(i+1, n));
        }

        ll yoko = (max_x - min_x + 1);
        ll tate = (max_y - min_y + 1);
        ll s = yoko * tate;
        if(s==n-1) s += min(yoko, tate);
        dbg(yoko, tate);
        chmin(ans,s);
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


