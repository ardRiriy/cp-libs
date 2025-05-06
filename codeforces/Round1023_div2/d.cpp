#include <bits/stdc++.h>
#include "input.hpp"

using namespace std;

// using namespace atcoder;

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

void dfs(vector<vector<int>>& g, vll& dist, int start) {
    queue<int> que;
    que.push(start);
    dist[start]=0;

    while(!que.empty()){
        auto p = que.front();
        que.pop();

        for(auto ni: g[p]){
            if(dist[ni]!=inf) continue;
            dist[ni]=dist[p]+1;
            que.push(ni);
        }
    }

    return;
}

void dfs2(vector<vector<int>>& g, vll& dist, int start, vector<ll>& from) {
    queue<int> que;
    que.push(start);
    dist[start]=0;
    from[start]=-1;

    while(!que.empty()){
        auto p = que.front();
        que.pop();

        for(auto ni: g[p]){
            if(dist[ni]!=inf) continue;
            from[ni]=p;
            dist[ni]=dist[p]+1;
            que.push(ni);
        }
    }

    return;
}

// 距離, 最遠点
pair<ll,int> dfs3(int p, vector<vector<int>>& g, vector<bool>&seen, vector<vector<pair<ll,int>>>& g2){
    seen[p] = true;
    pair<ll, int>res={0,p};
    for(auto ni: g[p]) {
        if(seen[ni]) continue;
        auto ret = dfs3(ni,g,seen,g2);
        g2[p].push_back(ret);
        if(res.first<ret.first) {
            res=ret;
        } else if(res.first==ret.first){
            if(res.second<ret.second){
                res=ret;
            }
        }
    }

    res.first+=1;
    res.second=p;
    return res;
}

void dfs4(int p, pair<ll,int> ue, vector<vector<pair<ll,int>>>& g2, vll& dist) {
    assert(p!=-1);
    if(p!=0){
        g2[p].push_back(ue);
    }

    sort(all(g2[p]));
    reverse(all(g2[p]));

    auto nxt = {0,-1};
    if(g2[p].size()>1){
        auto nxt = g2[p][1];
    }
    nxt.first++;
    nxt.second=-1;
    if(g2[p][0].second!=-1){
        dfs4(g2[p][0].second, nxt, g2, dist);
    }

    nxt = g2[p][0];
    nxt.first++;
    nxt.second=-1;
    rep2(i,1,g2.size()) {
        if(g2[p][i].second != -1) {
            dfs4(g2[p][i].second, nxt, g2, dist);
        }
    }
}

void solve() {
    int n; cin >> n;
    auto g = graph_IN(n,n-1);

    vector<tuple<ll,int,int>> ans;
    vector<vector<pair<ll,int>>> g2(n);
    vector<bool> seen(n,false);
    vll dist(n,inf);

    dfs3(0,g,seen,g2);
    dfs4(0,{-1,-1},g2,dist);
    assert(false);

    ll maxDist = -1;
    int p1 = -1;
    per(i,n){
        if(dist[i]==-1) continue;
        if(chmax(maxDist, dist[i])) {
            p1 = i;
        }
    }


    // dfs(g,dist,0);
    // int p1=-1;
    // {
    //     ll mx=-1;
    //     per(i,n){
    //         if(chmax(mx,ll(dist[i]))) p1=i;
    //     }
    // }

    fill(all(dist),inf);
    dfs(g,dist,p1);
    int p2=-1;
    {
        ll mx=-1;
        per(i,n){
            if(chmax(mx,dist[i])) p2=i;
        }
    }

    ans.push_back({dist[p2]+1, max(p1,p2), min(p1,p2)});

    
    fill(all(dist),inf);
    vll from(n,inf);
    dfs2(g,dist,p2, from);

    int cur = p1;
    while(cur!=-1){
        int prev = cur;
        cur=from[cur];
        from[prev] = -1;
    }

    // cerr << p1 << " " << p2 << '\n';
    // rep(i,n) {
    //     cerr << from[i] << ' ';
    // }
    // cerr << '\n';


    rep(i,n){
        if(from[i]==-1) continue;
        if(from[from[i]]!=-1) continue;

        int t1 = i;
        int cur = i;
        ll cnt = 1;
        while(true) {
            from[cur]=-1;
            int nxt = -1;
            for(auto ni: g[cur]) {
                if(from[ni]==cur)nxt=ni;
            }
            if(nxt==-1)break;
            cur = nxt;
            cnt++;
        }
        ans.push_back({cnt,max(t1,cur),min(t1,cur)});
    }
    sort(all(ans), [](const tuple<ll,int,int>&a, const tuple<ll,int,int>&b){
        auto [ad, au, av] = a;
        auto [bd, bu, bv] = b;
        if(ad==bd) {
            if(au==bu){
                return av>bv;
            } else {
                return au>bu;
            }
        } else {
            return ad>bd;
        }
    });

    rep(i,ans.size()){
        auto [d,u,v]=ans[i];
        cout << d << ' ' << u+1 << ' ' << v+1 << ((i+1==ans.size())?'\n':' ');
    }
}

int main() {
    cin.tie(0);
    ios::sync_with_stdio(false);
    int t=1;
    cin >> t;
    while(t--)solve();
}


