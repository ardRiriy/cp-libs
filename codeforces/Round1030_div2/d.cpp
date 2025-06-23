#include <bits/stdc++.h>
#include "input.hpp"

using namespace std;
// using namespace atcoder;

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

void solve() {
    int n, k; cin >> n >> k;
    dbg(n,k);
    auto p = i64_vec_IN(n);
    auto d = i64_vec_IN(n);

    // i, j, k := iに(kで割ったあまり)jで到達して、(k=0)?(左向き):(右向き)を向いていたときの状態
    // -1: 未訪問
    // 0: そこから始めて外に出られる
    // 1: otherwise
    vector seen(n, vector(k, vector(2, -1)));

    rep(i,n) rep(j,k) rep(l, 2) {
        if(seen[i][j][l] != -1) continue;
        
        vector<tuple<int,int,int>> track;

        ll state = -1;
        ll pos = i;
        ll tmod = j;
        ll rr = l;
        seen[pos][tmod][rr] = -2;
        track.push_back({pos,tmod,rr});
        while(state == -1) {

            // 反転チェック
            if(d[pos] == tmod) rr = 1-rr;

            // Y/N チェック
            if((pos == 0 && rr == 1) || (pos == n-1 && rr == 0)) {
                state = 0;
                break;
            }

            // 移動
            ll np = (rr == 0) ? pos+1 : pos-1;
            tmod = (tmod + abs(p[np] - p[pos])) % k;
            pos = np;
            if(seen[pos][tmod][rr] != -1) state = seen[pos][tmod][rr];
            else {
                track.push_back({pos,tmod,rr});
                seen[pos][tmod][rr] = -2;
            }
        }
        if (state == -2) state = 1;
  
        for(auto [ii, jj, kk]: track) {
            seen[ii][jj][kk] = state;
        }
    }

    rep(i,n) rep(j,k) rep(l, 2) cerr << seen[i][j][l] << '\n';


    int q; cin >> q;
    rep(i, q) {
        ll place; cin >> place;
        auto itr = lower_bound(all(p), place);
        if(itr == p.end()) {
            cout << "YES\n";
            continue;
        }
        ll idx = itr - p.begin();
        ll val = *itr;
        ll addiy = abs(val - place) % k;
        dbg(idx, val, addiy);
        if(seen[idx][addiy][0] == 0) cout << "YES\n";
        else cout << "NO\n";
    }

}

int main() {
    // cin.tie(0);
    // ios::sync_with_stdio(false);
    int t=1;
    cin >> t;
    while(t--)solve();
}


