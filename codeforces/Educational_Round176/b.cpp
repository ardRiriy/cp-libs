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
    int N, K; cin >> N >> K;
    vll a(N); rep(i, N) cin >> a[i];

    ll cur = 0;
    vector<vector<vector<ll>>> dp(2, vvll(K+1, vll(2, -inf)));
    vvll v(K+1, vll(2, -inf));

    dp[cur][0][0] = 0;
    rep(i, N) {

        ll nxt = 1-cur;
        rep(j, K+1) {

            if(i==0||i==N-1||(1<=j && j <= K-1))
                chmax(dp[nxt][j][1], dp[cur][j][0] + a[i]);

            rep(k, 2) {
                if(dp[cur][j][k] == -inf) continue;

                chmax(dp[nxt][j][k], dp[cur][j][k]);
                if(j<K) chmax(dp[nxt][j+1][k], dp[cur][j][k] + a[i]);   
            }            
        }
        dp[cur] = v;

        cur = nxt;
    }

    cout << dp[cur][K][1] << '\n';
}

int main() {
    cin.tie(0);
    ios::sync_with_stdio(false);
    int t;
    cin >> t;
    while(t--)solve();
}