#include <bits/stdc++.h>
#include "input.hpp"

using namespace std;
// using namespace atcoder;

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

/*

    dbg(g);

    ll ans = inf;
    do{
        ll cnt = 0;
        rep(i,n) {
            rep(j,n) {
                if(i==j) continue;
                if(abs(j-i)==1) {
                    if(!g[idxs[i]].contains(idxs[j]))cnt++;
                } else {
                    if(g[idxs[i]].contains(idxs[j]))cnt++;
                }
            }
        }
        if(chmin(ans,cnt)) dbg(idxs,cnt);
    } while (next_permutation(all(idxs)));
    
    cout << ans << '\n';
*/

ll dfs(int p, vector<vector<int>>& group, vector<set<int>>&g) {
    if(p==g.size()) {
        ll res = 0;
        rep(idx,group.size()) if(group[idx].size()<=2) return inf;
        rep(idx,group.size()) {
            int n = group[idx].size();
            vector<int> indicate(n,0);
            iota(all(indicate), 0);
            ll k = inf;
            do {
                ll cnt = 0;
                rep(i,n) {
                    ll c1 = g[group[idx][indicate[i]]].size();

                    if(!g[group[idx][indicate[i]]].contains(group[idx][indicate[(i+1)%n]]))c1++;
                    else c1--;
                    if(!g[group[idx][indicate[i]]].contains(group[idx][indicate[(i-1+n)%n]]))c1++;
                    else c1--;
                    // 2-addは
                    cnt+=c1;
                }
                if(chmin(k,cnt));
            } while(next_permutation(all(indicate)));
            res += k;
        }

        return  res/2;
    }

    ll res = inf;

    // add
    rep(i,group.size()) {
        group[i].emplace_back(p);
        ll ret = dfs(p+1, group,g);
        chmin(res,ret);
        group[i].pop_back();
    }

    group.push_back({p});
    ll ret=dfs(p+1, group,g);
    chmin(res,ret);
    group.pop_back();
    return res;
}

void solve() {
    int n, m; cin>>n>>m;
    //auto g = graph_IN(n,m);
    vector<set<int>> g(n);
    rep(i,m) {
        int u,v;cin>>u>>v;
        u--;v--;
        g[u].insert(v);
        g[v].insert(u);
    }
    vector<vector<int>> group;
    ll ans = dfs(0,group,g);
    cout << ans << '\n';

}

int main() {
    cin.tie(0);
    ios::sync_with_stdio(false);
    int t=1;
    //cin >> t;
    while(t--)solve();
}


