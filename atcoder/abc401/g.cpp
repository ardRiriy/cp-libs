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

#define rep(i,n) for (int i=0;i<n;++i)
#define per(i,n) for(ll i=n-1;i>=0;--i)
#define rep2(i,a,n) for (ll i=a;i<n;++i)
#define per2(i,a,n) for (ll i=n-1;i>=a;--i)


template<class T>bool chmax(T &a, const T &b) { if (a<b) { a=b; return true; } return false; }
template<class T>bool chmin(T &a, const T &b) { if (b<a) { a=b; return true; } return false; }

ll dx[] = {1, 0, -1, 0, -1, 1, -1, 1};
ll dy[] = {0, 1, 0, -1, -1, 1, 1, -1};

using ld = long double;
const int inf_i32 = 1<<30;

void bfs(vector<vector<int>>& g, vector<int>& dist, int n) {
    dist[2*n] = 0;
    queue<int> que;
    que.push(2*n);

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

vector<int> dfs(vector<vector<int>>& g, vector<int>& dist, int n) {
    vector<int> parent(g.size(), -1);
    parent[2*n] = inf_i32;
    stack<int> stk;
    stk.push(2*n);
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
    int n; cin>>n;
    vector<ld> sx(n), sy(n); rep(i,n) cin>>sx[i]>>sy[i];
    vector<ld> gx(n), gy(n); rep(i,n) cin>>gx[i]>>gy[i];

    vector<pair<ld, pair<int, int>>> vec;
    vector<vector<ld>> dist(n, vector<ld>(n));
    rep(i,n) rep(j,n) {
        dist[i][j] = hypot(sx[i]-gx[j], sy[i]-gy[j]);
        vec.push_back({dist[i][j], {i,j}});
    }
    sort(all(vec), [](const pair<ld, pair<int,int>>& a, pair<ld, pair<int,int>>& b) {
        return a.first < b.first;
    });


    int ng = -1;
    int ok = vec.size();

    // from: [0, n), to: [n, 2*n), 始点: 2*n 終点: 2*n+1
    vector<vector<int>> g(2*n+2, vector<int>(2*n+2, 0));
    vector<int> di(2*n+2);

    while(ok-ng>1){
        int mid=(ok+ng)>>1;
        cerr << mid << '\n';

        fill(g[2*n].begin(), g[2*n].begin()+n, 1); // 始点→from
        rep2(i, n, 2*n) {
            g[i][2*n+1] = 1; // to→終点
        }
        // vec[mid]以下の長さの辺からなるグラフを構築
        rep(i, mid+1) {
            auto [_, p] = vec[i];
            auto [ii, jj] = p;
            g[ii][n+jj] = 1;
        }


        while(true) {
            fill(all(di), inf_i32);
            bfs(g, di, n);
            if(di[2*n+1]==inf_i32) break;
            while(true){
                auto res = dfs(g,di,n);
                if(res.empty())break;

                int flow = inf_i32;
                rep(i,res.size()-1) chmin(flow, g[res[i]][res[i+1]]);
                rep(i,res.size()-1){
                    g[res[i]][res[i+1]] -= flow;
                    g[res[i+1]][res[i]] += flow;
                }
            }
        }
        ll sum = 0;
        rep(i,n){
            sum+=g[2*n][i];
        }

        if(sum==0)ok=mid;
        else ng=mid;
        fill(all(g), vector<int>(2*n+2,0));

    }
    cerr << ok << "\n";
    cout << fixed << setprecision(15) << vec[ok].first << '\n';
}

int main() {
    cerr << fixed << setprecision(15);
    cin.tie(0);
    ios::sync_with_stdio(false);
    int t=1;
    //cin >> t;
    while(t--)solve();
}


