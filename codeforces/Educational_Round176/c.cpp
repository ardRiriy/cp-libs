#include <bits/stdc++.h>
using namespace std;

#define all(v) v.begin(),v.end()
#define resort(v) sort(v.rbegin(),v.rend())
using ll = long long;
using ull = unsigned long long;
using vll=vector<ll>;
using vvll = vector<vector<ll>>;
using P = pair<ll,ll>;
using vp=vector<pair<ll, ll>>;

const ll inf=1ll<<60;
#define mod10 (ll)1e9+7
#define mod99 (ll)998244353
const double PI = acos(-1);

#define rep(i,n) for (ll i=0;i<n;++i)
#define per(i,n) for(ll i=n-1;i>=0;--i)
#define rep2(i,a,n) for (ll i=a;i<n;++i)
#define per2(i,a,n) for (ll i=n-1;i>=a;--i)


template<class T>bool chmax(T &a, const T &b) { if (a<b) { a=b; return true; } return false; }
template<class T>bool chmin(T &a, const T &b) { if (b<a) { a=b; return true; } return false; }

ll dx[] = {1, 0, -1, 0, -1, 1, -1, 1};
ll dy[] = {0, 1, 0, -1, -1, 1, 1, -1};

void solve() {
    int n, m; cin >> n >> m;
    vll a(m); rep(i, m) cin >> a[i];

    sort(all(a));

    ll ans = 0;
    rep2(i, 1, n) {
        ll ok = m;
        ll ng = -1;
        while(abs(ok-ng)>1) {
            ll mid = (ok+ng)>>1;
            if(a[mid]>=n-i) ok = mid;
            else ng = mid;
        }
        ll l = ok;
        ll u = m-ok;

        ok = m;
        ng = -1;
        while(abs(ok-ng)>1) {
            ll mid = (ok+ng)>>1;
            if(a[mid] >= i) {
                ok = mid;
            } else {
                ng = mid;
            }
        }

        ll r = ok;
        ll v = m-ok;
        ll d = m-max(l, r);
        ans += (m-ok)*v - d;
    }

    cout << ans << '\n';
}

int main() {
    cin.tie(0);
    ios::sync_with_stdio(false);
    int t;
    cin >> t;
    while(t--)solve();
}