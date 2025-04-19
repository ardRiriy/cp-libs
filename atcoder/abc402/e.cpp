#include <bits/stdc++.h>
#include "input.hpp"

using namespace std;

// using namespace atcoder;

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
    int n, x; cin >> n >> x;
    vector<long double> s(n), p(n);
    vector<int> c(n);
    rep(i,n) cin>>s[i]>>c[i]>>p[i];
    rep(i,n) p[i] = p[i]/100.0;
    vector<vector<long double>> dp(1<<n, vector<long double>(x+1, 0.0));
    rep(j,x+1) {
        per(i,(1<<n)) {
            long double exp = 0;
            rep(k,n) {
                int ni = i | (1<<k);
                if(i==ni) continue;
                if(j<c[k]) continue;
                long double val = p[k]*(s[k]+dp[ni][j-c[k]]) + (1.0-p[k])*dp[i][j-c[k]];
                chmax(exp, val);
            }
            dp[i][j] = exp;
        }
    }


    cout << fixed << setprecision(20) << dp[0][x] << '\n';
}

int main() {
    cin.tie(0);
    ios::sync_with_stdio(false);
    int t=1;
    //cin >> t;
    while(t--)solve();
}
