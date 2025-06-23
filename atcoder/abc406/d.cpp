#include <bits/stdc++.h>
#include "input.hpp"

using namespace std;
// using namespace atcoder;

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
    int h, w, n; cin >> h >> w >> n;
    vvll i_garb(max(h,w)); // i に対するj
    vvll j_garb(max(h,w)); // j に対するi
    int x,y;
    rep(i,n) {
        cin >> x >> y;
        x--; y--;
        i_garb[x].push_back(y);
        j_garb[y].push_back(x);
    }

    set<int> deleted_i, deleted_j;
    int q; cin >> q;
    rep(qi, q) {
        int t, p; cin >> t >> p;
        p--;
        int cnt = 0;

        if(t==1) {
            // iに対するjを全て削除
            if(deleted_i.contains(p)) {
                // 削除済み
                cout << "0\n";
                continue;
            }
            for(auto yi: i_garb[p]) {
                if(!deleted_j.contains(yi)) {
                    cnt++;
                }
            }
            deleted_i.insert(p);
        } else {
            if(deleted_j.contains(p)) {
                cout << "0\n";
                continue;
            }
            for(auto xi: j_garb[p]) {
                if(!deleted_i.contains(xi)) {
                    cnt++;
                }
            }
            deleted_j.insert(p);
        }
        cout << cnt << '\n';
    }

}

int main() {
    cin.tie(0);
    ios::sync_with_stdio(false);
    int t=1;
    //cin >> t;
    while(t--)solve();
}


