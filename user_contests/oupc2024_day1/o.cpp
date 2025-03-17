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
    ll s = 0;
    vll x(n), h(n),c(n);
    rep(i, n) cin >> x[i];
    rep(i, n) cin >> h[i];
    rep(i, n) { cin >> c[i]; s += c[i]; }
    
    if(n==1) {
        cout << "0\n";
        return;
    }

    vector<pair<int, int>> v;
    rep(i, n) rep2(j, i+1, n) v.push_back({i, j});
    stable_sort(all(v), [&](const P & a, const P & b) {
        // 傾きが小さい順に並べる
        auto [xi, xj] = a;
        auto [yi, yj] = b;

        ll l = (h[xj] - h[xi]) * (x[yj] - x[yi]);
        ll r = (x[xj] - x[xi]) * (h[yj] - h[yi]);
        if(l == r) return xj < yj;
        else return l < r;
    });

    vll maximam(n, 0);
    rep(i, n) maximam[i] = c[i];
    for(auto [i, j]: v) {
        chmax(maximam[j], maximam[i] + c[j]);
    }

    ll ans = 0;
    rep(i, n) chmax(ans, maximam[i]);
    cout << s - ans << '\n';
}

int main() {
    cin.tie(0);
    ios::sync_with_stdio(false);
    solve();
}
