#include <bits/stdc++.h>
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
S op(S a, S b) { return max(a, b); }
S e() { return -1e18; }

void solve() {
    int h, w, n; cin >> h >> w >> n;
    vvll g(h);
    int r, c;
    rep(i,n) {
        cin >> r >> c;
        r--; c--;
        g[r].emplace_back(c);
    }

    map<P, P> ans;
    vp next(w, {h-1,w-1});

    segtree<S,op,e> seg(w);
    rep(i,w) seg.set(i,0);

    per(i,h) {
        if(g[i].empty()) continue;
        sort(all(g[i]));
        reverse(all(g[i]));

        for(auto wi: g[i]) {
            ll val = seg.prod(wi,w);
            // 最大値がvalとなるような右端を探索
            ll ng = wi-1;
            ll ok = w;
            while(abs(ok-ng)>1) {
                ll mid = (ng+ok)>>1;
                dbg(wi,mid);
                if(seg.prod(wi, mid) == val) {
                    ok = mid;
                } else {
                    ng = mid;
                }
            }
            ans[{i,wi}] = next[ok-1];
            next[wi] = {i,wi};
            seg.set(wi, val+1);
        }
    }

    ll val = seg.prod(0,w);
    ll ng = 0;
    ll ok = w;
    while(abs(ok-ng)>1) {
        ll mid = (ng+ok)>>1;
        if(seg.prod(0, mid) == val) {
            ok = mid;
        } else {
            ng = mid;
        }
    }

    string move_str= "";
    P cur = next[ok-1];
    // 0,0 ~ cur
    rep(_, cur.first) {
        move_str += 'D';
    }
    rep(_, cur.second) {
        move_str += 'R';
    }

    while (!(cur.first == h-1 && cur.second == w-1)){
        P to = ans[cur];
        rep(_, to.first - cur.first) {
            move_str += 'D';
        }
        rep(_, to.second - cur.second) {
            move_str += 'R';
        }
        cur = to;
    }
    cout << val << '\n' << move_str << '\n';
}

int main() {
    cin.tie(0);
    ios::sync_with_stdio(false);
    int t=1;
    //cin >> t;
    while(t--)solve();
}


