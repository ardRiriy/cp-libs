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
    int n, m;
    string s;
    cin >> n >> m >> s;
    ll a[n][m];
    ll b[n][m];
    rep(i, n) rep(j, m) {
        cin >> a[i][j];
        b[i][j] = a[i][j];
    }

    vll row_sum(n, 0), col_sum(m, 0);

    rep(i, n) {
        rep(j, m) {
            row_sum[i] += a[i][j];
            col_sum[j] += a[i][j]; 
        } 
    }

    int pi=0, pj=0;
    for (char c: s) {
        if (c == 'D') {
            b[pi][pj] = -row_sum[pi];
            row_sum[pi] += b[pi][pj];
            col_sum[pj] += b[pi][pj];
            pi += 1;
        } else {
            b[pi][pj] = -col_sum[pj];
            row_sum[pi] += b[pi][pj];
            col_sum[pj] += b[pi][pj];
            pj += 1;
        }
        if (c == 'D') {
        }
    }
    b[n-1][m-1] = -row_sum[n-1];
    assert(col_sum[m-1] + b[n-1][m-1] == 0);

    rep(i, n) rep(j, m) cout << b[i][j] << ((j+1 == m) ? '\n' : ' ');
}


int main() {
    cin.tie(0);
    ios::sync_with_stdio(false);
    int t;
    cin >> t;
    while(t--)solve();
}