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
    vvll g(n, vll(n, 0)); // g_i,j := 頂点iから頂点jへ向かう容量の最大
    int u, v, w;
    rep(_,m){
        cin >> u >> v >> w;
        g[u][v] += w;
    }

    ll sum = 0;
    rep(i,n) sum+=g[0][i];

    vector<pair<int, int>> tracking;
    vector<bool> seen(n, false);
    while(true) {
        stack<pair<int, int>> stk;
        
        // dfs
        stk.push({0, 1<<30});
        
        while(!stk.empty() && !seen[n-1]) {
            auto [p, flow] = stk.top(); stk.pop();

            if(p<0||p>=n) {
                // 帰りがけ
                assert(!tracking.empty());
                tracking.pop_back();
            } else {
                if(seen[p]) { continue; }
                seen[p] = true;
    
                // 行きがけ
                tracking.push_back({p, flow});
                stk.push({~p, 0});

                rep(i, n) {
                    if(g[p][i] > 0 && !seen[i]) {
                        stk.push({ i, g[p][i] });
                    }
                }
            }
        }

        if(tracking.empty()) break; // 路が存在しない

        int flow = 1<<30;
        for(auto[_, s]: tracking) {
            chmin(flow, s);
        }
        
        rep(i, tracking.size()-1) {
            auto [from, _] = tracking[i];
            auto [to, __] = tracking[i+1];

            g[from][to] -= flow;
            g[to][from] += flow;

            assert(g[from][to] >= 0);
        }
        tracking.clear();
        fill(all(seen), false);
    }

    ll d = 0;
    rep(i,n) d += g[0][i];
    cout << sum - d << '\n';
}

int main() {
    cin.tie(0);
    ios::sync_with_stdio(false);
    int t=1;
    //cin >> t;
    while(t--)solve();
}


