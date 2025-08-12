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

const int inf=1ll<<30;
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

int n = 26;
int solve(){
    int h, w; cin >> h >> w;
    vector<string> s(h); rep(i,h) cin >> s[i];

    auto checker = [&](char c) -> vll {
        // 上に被っているものの配列を返す
        ll l=inf, r=-1, u=inf, d=-1;
        rep(i,h) {
            rep(j,w) {
                if(s[i][j] == c) {
                    chmin(l,j);
                    chmax(r,j);
                    chmin(u,i);
                    chmax(d,i);
                }
            }
        }
        if(l==inf) return {};

        vll res;
        rep2(i,u,d+1) {
            rep2(j,l,r+1) {
                if(s[i][j] == '.') return {-1}; // 構成不可能なときはこれ
                else if(s[i][j] != c) res.emplace_back(s[i][j] - 'A');
            }
        }
        sort(all(res));
        res.erase(unique(all(res)),res.end());
        return res;
    };

    vvll g(n);
    vll ins(n,0);

    rep(i,n) {
        g[i] = checker('A'+i);
        dbg(i, g[i]);
        if(g[i].size()>0 && g[i][0]==-1) return 1;
        for(auto ni: g[i]) ins[ni]++;
    }

    queue<int> que;
    vector<bool> seen(26,false);
    rep(i,n) {
        if(ins[i] == 0) {
            que.push(i);
            seen[i] = true;
        }
    }
    while(!que.empty()) {
        auto p = que.front();
        que.pop();
        for(auto ni: g[p]) {
            if(--ins[ni] == 0) {
                seen[ni] = true;
                que.push(ni);
            }
        }
    }

    bool all_seen = true;
    rep(i,n) all_seen = all_seen && seen[i];
    return !all_seen;
}

int main() {
    cin.tie(0);
    ios::sync_with_stdio(false);
    int t; cin >> t;
    while(t--) {
        // 0ならsafe
        if(solve()==0) {
            cout << "SAFE\n";
        } else {
            cout << "SUSPICIOUS\n";
        }
    }
}

