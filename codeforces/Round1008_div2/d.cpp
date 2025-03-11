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
    int n; cin >> n;
    vector<vector<pair<char, ll>>> v(n, vector<pair<char, ll>>(2));
    rep(i, n) {
        char t1, t2;
        ll v1, v2;
        cin >> t1 >> v1 >> t2 >> v2;

        v[i][0] = { t1, v1 };
        v[i][1] = { t2, v2 };
    }

    vvll inc(n+1, vll(2, 1));
    per(i, n) {
        rep(j, 2) {
            if(v[i][j].first == 'x') {
                inc[i][j] = v[i][j].second - 1 + inc[i+1][j];
            } else {
                inc[i][j] = inc[i+1][j];
            }
        }
    }


    ll cur[2] = {1, 1};
    rep(i, n) {
        // 増分を計算
        ll inc_val = 0;
        rep(j, 2) {
            if(v[i][j].first == '+') {
                inc_val += v[i][j].second;
            } else {
                inc_val += cur[j] * (v[i][j].second-1);
            }
        }
        if (inc[i+1][0] > inc[i+1][1]) {
            cur[0] += inc_val;
        } else {
            cur[1] += inc_val;
        }
        cout << cur[0] << ' ' << cur[1] << '\n';
    }
    cout << cur[0] + cur[1] << '\n';
}

int main() {
    cin.tie(0);
    ios::sync_with_stdio(false);
    int t;
    cin >> t;
    while(t--)solve();
}