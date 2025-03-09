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


int primes[12] = {2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37};
ll dfs(int p, vvll& g, vector<bool>& seen, int count[], int dept, int a[]) {
    seen[p] = true;
    ll idx = 13;
    rep(i, 12) {
        if(a[p] % primes[i] == 0) count[i]++;
        if(count[i] != dept) idx = min(idx, i);
    }

    ll res = primes[idx];
    //cerr << p << ": " << res << '\n';

    for(ll ni: g[p]) {
        if(seen[ni]) continue;
        res += dfs(ni, g, seen, count, dept+1, a);
    }


    rep(i, 12) {
        if(a[p] % primes[i] == 0) count[i]--;
    }

    return res;
}

void solve() {
    int n; cin >> n;
    int a[n]; rep(i, n) cin >> a[i];
    vvll g(n);
    rep(i, n-1) {
        int u, v; cin >> u >> v;
        u--;v--;
        g[u].push_back(v);
        g[v].push_back(u);
    }

    vll ans;
    int count[12];
    rep(i, 12) count[i] = 0;
    rep(i, n) {
        vector<bool> seen(n, false);
        ans.push_back(dfs(i, g, seen, count, 1, a));
    }
    rep(i, n) cout << ans[i] << ((i+1==n)? '\n' : ' ');
}

int main() {
    cin.tie(0);
    ios::sync_with_stdio(false);
    int t;
    cin >> t;
    while(t--) solve();
}