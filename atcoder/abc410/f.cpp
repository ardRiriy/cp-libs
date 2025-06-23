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

#define rep(i,n) for (int i=0;i<(n);++i)
#define per(i,n) for(int i=(n)-1;i>=0;--i)
#define rep2(i,a,n) for (int i=(a);i<(n);++i)
#define per2(i,a,n) for (int i=(n)-1;i>=(a);--i)


template<class T>bool chmax(T &a, const T &b) { if (a<b) { a=b; return true; } return false; }
template<class T>bool chmin(T &a, const T &b) { if (b<a) { a=b; return true; } return false; }

ll dx[] = {1, 0, -1, 0, -1, 1, -1, 1};
ll dy[] = {0, 1, 0, -1, -1, 1, 1, -1};

unordered_map<int, int> cnt;
/* √300000 <= 1000 */
void solve() {
    int h, w; cin >> h >> w;
    vector<string> s(h);
    rep(i,h) cin >> s[i];

    bool tr = (h > w);
    int nh = tr ? w : h;
    int nw = tr ? h : w;

    vector<vector<int>> csum(nh + 1, vector<int>(nw, 0));

    if (tr) {
        rep(i, h) rep(j, w) {
            int v = (s[i][j] == '.') ? -1 : 1;
            csum[j + 1][i] = csum[j][i] + v;
        }
        swap(h, w);
    } else {
        rep(i, h) rep(j, w) {
            int v = (s[i][j] == '.') ? -1 : 1;
            csum[i + 1][j] = csum[i][j] + v;
        }
    }

    ll ans = 0;
    rep(d, h) {
        rep2(u, d+1, h+1) {
            // [d, u)の区間を取るときに、右に尺取法
            cnt.clear();

            int cur = 0;
            rep(i, w) {
                ans += cnt[cur];
                cnt[cur]++;
                cur += csum[u][i] - csum[d][i];
            }
            ans +=cnt[cur];
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
