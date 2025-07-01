#include <bits/stdc++.h>
#include "input.hpp"
#include "atcoder/dsu.hpp"
using namespace std;
using namespace atcoder;

#ifdef ADRY
#include <dbg.h>
#else
// DO NOTHING
#define dbg(...)
#endif

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

#define rep(i,n) for (ll i=0;i<(n);++i)
#define per(i,n) for(ll i=(n)-1;i>=0;--i)
#define rep2(i,a,n) for (ll i=(a);i<(n);++i)
#define per2(i,a,n) for (ll i=(n)-1;i>=(a);--i)


template<class T>bool chmax(T &a, const T &b) { if (a<b) { a=b; return true; } return false; }
template<class T>bool chmin(T &a, const T &b) { if (b<a) { a=b; return true; } return false; }

ll dx[] = {1, 0, -1, 0, -1, 1, -1, 1};
ll dy[] = {0, 1, 0, -1, -1, 1, 1, -1};

void solve() {
    int n,m;cin >> n >> m;
    vector<pair<int,int>> edges(m);

    rep(i,m) {
        cin >> edges[i].first >> edges[i].second;
        edges[i].first--;
        edges[i].second--;
    }

    vector<set<int>> group(n), graph(n);
    for(auto [u,v]: edges) {
        graph[u].insert(v);
        graph[v].insert(u);
    }
    rep(i,n) {
        group[i].insert(i);
    }

    vector<int> parent(n);
    iota(all(parent), 0);

    ll ans = m;
    int q; cin >> q;

    auto zaatu = [&](auto self, int u) -> int {
        if(parent[u]==u) return u;
        return parent[u] = self(self,parent[u]);
    };

    rep(i,q) {
        int x; cin >> x;
        auto [a,b] = edges[x-1];
        int u = parent[a];
        int v = parent[b];
        if(u==v) {
            cout << ans << '\n';
            continue;
        }

        // 頂点集合のマージ
        if(group[u].size()>group[v].size()) {
            swap(u,v);
        }
     
        for(auto to: graph[u]) {
            if(to==v) {
                continue;
            }
            if(graph[to].contains(v)) {
                graph[to].erase(u);
                ans--;
            } else {
                graph[to].erase(u);
                graph[to].insert(v);
                graph[v].insert(to);
            }
        }

        for(auto to: graph[u]) {
            if(to==v) {
                ans--;
                graph[v].erase(u);
            }
        }

        for(auto to: group[u]) {
            group[v].insert(to);
            parent[to] = v;
        }
     
        group[u].clear();
        graph[u].clear();

        dbg(group);
        dbg(graph);
        dbg("====");

        cout << ans << '\n';

    }
}


int main() {
    cin.tie(0);
    ios::sync_with_stdio(false);
    int t=1;
    //cin >> t;
    while(t--)solve();
}
