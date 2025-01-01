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

void warshall_floyd(vvll &dist) {
    ll v = dist.size();
    rep(k, v) {
        rep(i, v) {
            rep(j, v) {
                dist[i][j] = min(dist[i][j], dist[i][k] + dist[k][j]);
            }
        }
    }
}

void solve() {
    int n, m, q;
    cin >> n >> m;

    vvll dist(n, vll(n, inf));
    rep(i, n) dist[i][i] = 0;
    vector<vector<ll>> edge(m, vector<ll>(3));
    rep(i, m) {
        ll u, v, w;
        cin >> u >> v >> w;
        u--; v--;
        edge[i] = {u, v, w};
        chmin(dist[u][v], w);
        chmin(dist[v][u], w);
    }

    warshall_floyd(dist);

    cin >> q;
    rep(_, q) {
        int k;
        cin >> k;
        vll b(k);

        ll base = 0;
        rep(i, k) { 
            cin >> b[i]; 
            b[i]--; 
            base += edge[b[i]][2];
        }
        ll ans = inf;
        do {
            rep(i, 1<<k) {
                ll cnt = base;
                // 0ならu→v, 1ならv→u
                cnt += dist[0][edge[b[0]][i & 1]];
                rep(j, k-1) {
                    cnt += dist[edge[b[j]][1 - (i>>j)&1]][edge[b[j+1]][(i >> (j+1))&1]];
                }
                cnt += dist[edge[b[k-1]][1 - (i>>(k-1))&1]][n-1];
                chmin(ans, cnt);
            }
        } while(next_permutation(all(b)));

        cout << ans << "\n";
    }
} 

int main() {
    cin.tie(0);
    ios::sync_with_stdio(false);
    solve();
}