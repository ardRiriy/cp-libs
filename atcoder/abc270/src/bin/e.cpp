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
using djks=priority_queue<P, vp, greater<P>>;

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
    ll n, k; cin >> n >> k;
    ll mx = -1;
    vll a(n); rep(i, n) cin >> a[i], mx = max(mx, a[i]);

    // ok週したときにK以下となる
    ll ok = 0;
    ll ng = mx+1;

    while(abs(ok-ng)>1) {
        ll mid = (ok+ng)>>1;

        ll cnt = 0;
        rep(i, n) {
            cnt += min(a[i], mid);
        }
        if(cnt<=k) {
            ok=mid;
        } else {
            ng=mid;
        }
    }

    ll cnt = 0;
    rep(i, n) cnt += min(a[i], ok);
    
    ll left = k - cnt;
    rep(i, n) {
        if(left==0) break;
        if(a[i]-ok>0)a[i]--, left -= 1;
    }

    rep(i, n) cout << max(a[i]-ok, 0LL) << ((i+1==n)?'\n':' ');
}

int main() {
    cin.tie(0);
    ios::sync_with_stdio(false);
    int t=1;
    //cin >> t;
    while(t--)solve();
}


