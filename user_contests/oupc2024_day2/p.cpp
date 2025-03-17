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

ll dp(vll& v) {
    ll n = v.size();

    vvll dp(n+1, vll(2, 0));

    rep(i, n) {
        chmax(dp[i+1][0], dp[i][0]);
        chmax(dp[i+1][1], dp[i][0] + v[i]);
        chmax(dp[i+1][0], dp[i][1]);
    }
    return max(dp[n][0], dp[n][1]);
}

void solve() {
    ll n; cin >> n;


    vector<string> a(n); rep(i, n) cin >> a[i];
    vll b(n); rep(i,n) cin >> b[i];
    ll minus = 0;
    ll is = 0;
    rep(i, n) {
        if(a[i] == "i" || a[i] == "-i") {
            is += b[i];
        }
        if(a[i] == "-1" || a[i] == "-i") {
            minus += b[i];
        }
    }

    bool has_i = is % 2 == 1;
    minus += is / 2;
    bool has_minus = minus % 2 == 1;

    if(has_i) {
        if(has_minus) cout << "-i\n";
        else cout << "i\n";
    } else {
        if(has_minus) cout << "-1\n";
        else cout << "1\n";
    }
}

int main() {
    cin.tie(0);
    ios::sync_with_stdio(false);
    int t=1;
    //cin >> t;
    while(t--)solve();
}
