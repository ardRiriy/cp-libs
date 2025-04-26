#include <bits/stdc++.h>
#include <atcoder/all>

using namespace std;
using namespace atcoder;

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
    int n; cin >> n;
    vll a(n); rep(i,n)cin>>a[i];
    vll csum(n+1, 0); rep(i,n) csum[i+1] = csum[i] + a[i];
    vll value(n, 0);
    rep(i,n) {
        value[i] = 2*csum[i/2];
        if(i%2==1) {
            value[i] += a[i/2];
        }
    }

    vll dp(n+1, -1);
    dp[0] = 0;
    rep(i, n) {
        if(dp[i]==-1) continue; // ありえないはずだが、念の為
        rep(j,n+1) {
            if(i+j+1>n) break;
            chmax(dp[i+j+1], dp[i] + value[j]);
        }
    }
    cout << dp[n] << '\n';
}

int main() {
    cin.tie(0);
    ios::sync_with_stdio(false);
    int t=1;
    //cin >> t;
    while(t--)solve();
}


