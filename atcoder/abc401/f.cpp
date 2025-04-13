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

void bfs(vvll& g, vll& dist, ll s) {
    queue<ll> que;
    dist[s] = 0;
    que.push(s);
    while(!que.empty()) {
        ll p = que.front(); que.pop();
        for(auto ni: g[p]) {
            if(dist[ni] != -1) continue;
            dist[ni] = dist[p] + 1;
            que.push(ni);
        }
    }
}

void solve() {
    int n; cin >> n;
    vvll g(n);
    rep(i,n-1) {
        int u, v; cin >> u >> v; u--, v--;
        g[u].push_back(v);
        g[v].push_back(u);
    }

    vll dist_n(n, -1); 
    
    {
        vvll t_dn(3, vll(n, -1));
        vll t(4, 0);
        rep(idx, 3) {
            bfs(g, t_dn[idx], t[idx]);
            ll mx = -1;
            rep(i,n) if(chmax(mx, t_dn[idx][i])) t[idx+1] = i;
        }

        rep(i,n) dist_n[i] = max(t_dn[1][i], t_dn[2][i]);
    }

    int m; cin >> m;
    vvll h(m);
    rep(i,m-1) {
        int u, v; cin >> u >> v; u--; v--;
        h[u].push_back(v);
        h[v].push_back(u);
    }

    vll dist_m(m, -1);

    {
        vvll t_dn(3, vll(m, -1));
        vll t(4, 0);
        rep(idx, 3) {
            bfs(h, t_dn[idx], t[idx]);
            ll mx = -1;
            rep(i,m) if(chmax(mx, t_dn[idx][i])) t[idx+1] = i;
        }

        rep(i,m) dist_m[i] = max(t_dn[1][i], t_dn[2][i]);
    }

    sort(all(dist_m));
    vll csum(m+1, 0);
    rep(i,m) {
        csum[i+1] = csum[i] + dist_m[i];
    }

    ll d_max = 0;
    rep(i,n) chmax(d_max, dist_n[i]);
    rep(i,m) chmax(d_max, dist_m[i]);

    ll ans = 0;
    rep(i,n) {
        ll key = d_max - dist_n[i];
        auto itr = lower_bound(all(dist_m), key);
        int d = itr - dist_m.begin();
        ans += csum[m] - csum[d];
        ans += (dist_n[i]+1)*(m-d);

        ans += d_max * d;
    }
    cout << ans << '\n';
}

int main() {
    cin.tie(0);
    ios::sync_with_stdio(false);
    int t=1;
    //cin >> t;
    while(t--)solve();
}


