#include <bits/stdc++.h>
using namespace std;

#define int long long
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
    int n, m, t; cin >> n >> m >> t;
    int a[n]; rep(i, n-1) cin >> a[i]; 
    pair<int, int> b[m]; rep(i, m) cin >> b[i].first >> b[i].second;
    int bi = 0;
    rep(i, n-1) {
        if(bi < m && b[bi].first == i+1) {
            t += b[bi].second;
            bi += 1;
        }
        if(t<=a[i]) {
            cout << "No\n";
            return;
        }
        t -= a[i];
    }
    cout << "Yes\n";
}

signed main() {
    cin.tie(0);
    ios::sync_with_stdio(false);
    int t=1;
    //cin >> t;
    while(t--)solve();
}
