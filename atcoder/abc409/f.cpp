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
    int n, q; cin >> n >> q;
    vp pos(n);
    rep(i,n) {
        cin >> pos[i].first >> pos[i].second;   
    }

    priority_queue<tuple<ll, int,int>, vector<tuple<ll,int,int>>, greater<tuple<ll,int,int>>> pq;
    dsu uf(n+q);
    rep(i,n) {
        auto [x1, y1] = pos[i];
        rep2(j, i+1, n) {
            auto [x2, y2] = pos[j];
            ll d = abs(x1-x2)+abs(y1-y2);
            pq.push({d,i,j});
        }
    }

    dbg("===");
    int t;
    rep(_, q) {
        cin >> t;
        if(t==1) {
            ll ver_idx = pos.size();
            ll a, b; cin >> a >> b;

            rep(i,pos.size()) {
                auto [x,y] = pos[i];
                ll d = abs(x-a) + abs(y-b);
                dbg(i,ver_idx);
                pq.push({d,i,ver_idx});
            }

            pos.push_back({a,b});
        } else if(t==2) {
            ll res = -1;
            while(!pq.empty()) {
                auto [k, i, j] = pq.top();
                dbg(i,j);
                if(uf.same(i, j)) {
                    pq.pop();
                    continue;
                } else {
                    res = k;
                    break;
                }
            }
            if(res==-1) cout << "-1\n";
            else {
                cout << res << '\n';
                while(!pq.empty()) {
                    auto [w,i,j] = pq.top();
                    if(w!=res)  {
                        break;
                    } else {
                        pq.pop();
                        uf.merge(i,j);
                    }
                }
            }
        } else {
            int u, v;
            cin >> u >> v;
            u--, v--;
            dbg(u,v);
            if(uf.same(u,v)) cout << "Yes\n";
            else cout << "No\n";
        }
    }

}

int main() {
    // cin.tie(0);
    // ios::sync_with_stdio(false);
    int t=1;
    //cin >> t;
    while(t--)solve();
}


