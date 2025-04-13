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

const int inf_i32 = 1<<30;
void bfs(vector<vector<int>>& g, vector<int>& dist) {
    dist[0] = 0;
    queue<int> que;
    que.push(0);

    while(!que.empty()) {
        int p = que.front(); que.pop();
        rep(i,g[p].size()) {
            if(g[p][i] <= 0) continue;
            if(dist[i] != inf_i32) continue;
            dist[i] = dist[p]+1;
            que.push(i);
        }
    }
}

vector<int> dfs(vector<vector<int>>& g, vector<int>& dist) {
    vector<int> parent(g.size(), -1);
    parent[0] = inf_i32;
    stack<int> stk;
    stk.push(0);
    while (!stk.empty()){
        int p = stk.top(); stk.pop();
        rep(i, g.size()) {
            if(parent[i] != -1 || g[p][i] <= 0 || dist[p]+1 != dist[i] ) continue;
            parent[i] = p;
            stk.push(i);
        }
    }

    vector<int> res;
    if(parent[g.size()-1]!=-1) {
        int cur = g.size()-1;
        while (cur!=inf_i32){
            res.push_back(cur);
            cur = parent[cur];
        }
    }
    reverse(all(res));
    return res;
}

void solve() {
    int n, m; cin >> n >> m;
    vector<vector<int>> g(n, vector<int>(n, 0)); // g_i,j := 頂点iから頂点jへ向かう容量の最大
    int u, v, w;
    rep(_,m){
        cin >> u >> v >> w;
        g[u][v] += w;
    }

    ll sum = 0;
    rep(i,n) sum+=g[0][i];
    vector<int> dist(n, inf_i32);

    while(true) {
        bfs(g, dist);

        if(dist[n-1]==inf_i32) break;
        while (true){
            auto res = dfs(g, dist);


            if(res.empty()) break;
            int flow = inf_i32;
            rep(i,res.size()-1) {
                int from = res[i];
                int to = res[i+1];
                chmin(flow, g[from][to]);
            }
            rep(i, res.size()-1) {
                int from = res[i];
                int to = res[i+1];
                g[from][to] -= flow;
                g[to][from] += flow;
            }
        }
        fill(all(dist), inf_i32);
    }

    ll d = 0;
    rep(i,n) d+=g[0][i];
    cout << sum-d << "\n";
}

int main() {
    cin.tie(0);
    ios::sync_with_stdio(false);
    int t=1;
    //cin >> t;
    while(t--)solve();
}


