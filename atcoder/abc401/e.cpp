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
    vvll g(n);
    int u,v;
    rep(i,m) {
        cin >> u >> v;
        u--; v--;
        g[u].push_back(v);
        g[v].push_back(u);
    }

    priority_queue<ll, vll, greater<ll>> pq;
    vector<bool> seen(n, false);
    vector<bool> visited(n, false);
    vector<ll> ans(n, -1);
    pq.push(0);
    seen[0] = true;
    ll cur = 0;
    ll visited_max = -1;
    while(!pq.empty()) {
        ll p = pq.top(); pq.pop();
        visited[p] = true;
        chmax(visited_max, p);

        for(auto ni: g[p]) {
            if(seen[ni]) { continue; }
            seen[ni] = true;
            pq.push(ni);
        }

        // 正誤処理
        while(cur<n&&visited[cur]) {
            if(visited_max==cur) {
                ans[cur] = pq.size();
            } else {
                ans[cur] = -1;
            }
            cur++;
        }

    }
    rep(i,n) {
        cout << ans[i] << '\n';
    }
}

int main() {
    cin.tie(0);
    ios::sync_with_stdio(false);
    int t=1;
    //cin >> t;
    while(t--)solve();
}


