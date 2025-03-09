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
    int n;
    cin >> n;
    vll a(n); rep(i, n) cin >> a[i];
    if(n==2) {
        cout << max(a[0], a[1]) - min(a[0], a[1]) << '\n'; 
    } else if(n==3) {
        cout << max(max(a[1]-(a[0]+a[2]), a[0]), max(a[2], 0LL)) << '\n';
    } else {
        ll ans = 0;
        rep(i, n) {
            if(i==1) {
                chmax(ans, a[1]-a[0]);
            } else if (i+2==n){
                chmax(ans, a[n-2]-a[n-1]);
            } else {
                chmax(ans, a[i]);
            }
        }
        cout << ans << '\n';
    }
}

int main() {
    cin.tie(0);
    ios::sync_with_stdio(false);
    int t;cin >>t;
    while(t--) solve();
}