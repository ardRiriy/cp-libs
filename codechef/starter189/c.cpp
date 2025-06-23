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
    dbg("===========");
    int n; cin >> n;
    vll a = i64_vec_IN(n);

    ll ans = 0;
    rep(l, n) {
        rep2(x,1,n-l+1) {
            // [l, l+x)を判定
            ll cur = 0;
            bool creatable = true;
            rep2(i, l, l+x) {
                ll y = (a[i] % 2 == 0) ? a[i]/2-1 : a[i] / 2;
                // [0, y]が作れる
                dbg(a[i], y);
                if(cur<=y) {
                    cur++;
                } else if(cur<=a[i]) {
                    cur = a[i] + 1;
                } else {
                    creatable = false;
                }
            }
            if(creatable) {
                dbg(l, l+x);
                ans++;
            }
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


