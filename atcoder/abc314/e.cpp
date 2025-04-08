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
    int n, m; cin >> n >> m;
    vvll p(n);
    vll c(n);
    rep(i,n){
        cin >> c[i];
        int t; cin >> t;
        p[i].resize(t);
        rep(j,t){
            cin >> p[i][j];
        }
    }

    vector<long double> e(m, 1e18);
    per(i, m) {
        rep(j, n) {
            long double est = c[j];
            ll zero_cnt = 0;
            long double len = p[j].size();
            for(ll s: p[j]) {
                if(s==0) {
                    zero_cnt++;
                    continue;
                }
                if(s+i<m) {
                    est += e[s+i] / len;
                }
            }
            if(zero_cnt>0){
                est *= len;
                est /= (len - (long double)zero_cnt);
            }

            chmin(e[i], est);
        }
    }

    cout << e[0] << '\n';
}

int main() {
    cin.tie(0);
    ios::sync_with_stdio(false);
    cout << fixed << setprecision(15);
    int t=1;
    //cin >> t;
    while(t--)solve();
}


