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
    ll n, m; cin >> n >> m;
    map<ll, ll> map; // 位置xにたいして高さの2乗のmaxを持つ
    vector<ll> xs(n), rs(n);
    rep(i, n) cin >> xs[i];
    rep(i, n) cin >> rs[i];

    rep(i, n) {
        ll x = xs[i];
        ll r = rs[i];
        rep(k, r) {
            ll h = r*r - k*k;
            map[x-k] = max(map[x-k], h);
            map[x+k] = max(map[x+k], h);
        }

        // k==rの場合(0だけ)
        map[x-r] = max(map[x-r], 0LL);
        map[x+r] = max(map[x+r], 0LL); 
    }

    ll ans = 0;
    for(auto itr = map.begin(); itr != map.end(); itr++) {
        ll h = itr->second;
        // x*2 <= hなるxを二分探索
        ll ok = 0;
        ll ng = m+1;
        while(abs(ng-ok)>1) {
            ll mid = (ok+ng)>>1;
            if(mid*mid <= h) {
                ok = mid;
            } else {
                ng = mid;
            }
        }
        ans += ok*2+1;
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