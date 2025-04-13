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
    int n, m;cin>>n>>m;
    vvll a(n, vll(m));

    rep(i,n)rep(j,m) cin >> a[i][j];

    vector<vvll> target(4, vvll(n, vll(m)));

    ll idx = 0;
    for(int di: {-1, 1}) {
        for(int dj: {-1, 1}) {
            target[idx][0][0] = 0;
            rep(i, n) rep(j, m) {
                if(i==0&&j==0) { continue; }
                if(j == 0) {
                    // 上から
                    target[idx][i][j] = target[idx][i-1][j] + di;
                } else {
                    target[idx][i][j] = target[idx][i][j-1] + dj;
                }
            }
            idx++;
        }
    }

    ll ans = inf;
    rep(i,n){
        rep(j,m) {
            rep(k, 4) {
                ll cnt = 0;

                rep(ni, n) {
                    rep(nj, m) {
                        if(a[i][j] - a[ni][nj] == target[k][i][j] - target[k][ni][nj]) {

                        } else {
                            cnt++;
                        }
                    }
                }
                chmin(ans, cnt);

            }
        }
    }

    cout << ans << '\n';
}

int main() {
    cin.tie(0);
    ios::sync_with_stdio(false);
    int t;
    cin >> t;
    while(t--) solve();
    
}