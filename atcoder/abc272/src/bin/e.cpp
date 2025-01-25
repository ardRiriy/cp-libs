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
    ll a[n]; rep(i, n) cin >> a[i];

    vector<vector<ll>> v(m+1);
    rep(i, n) {
        if(a[i] >= ll(n)) continue;
        ll l = (a[i] >= 0) ? 1 : (-a[i]+i)/(i+1);

        ll r = min((ll(n)-a[i]+i)/(i+1), ll(m+1));
        rep2(j, l, r) {
            v[j].push_back(a[i] + (i+1)*j);
        }
    }

    rep2(i, 1, m+1) {
        sort(all(v[i]));
        int ans = 0;
        rep(j, v[i].size()) {
            if (ans == v[i][j]) ans++;
        }
        cout << ans << '\n';    
    }
}

int main() {
    cin.tie(0);
    ios::sync_with_stdio(false);
    int t = 1;
    // cin >> t;
    while(t--)solve();
}
