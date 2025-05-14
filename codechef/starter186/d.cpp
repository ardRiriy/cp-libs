#include <bits/stdc++.h>
#include "input.hpp"

using namespace std;
// using namespace atcoder;

#ifdef ADRY
#include <dbg.h>
#else
// DO NOTHING
#define dbg(x)
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
    int n, x, y; cin >> n >> x >> y;
    auto a = i64_vec_IN(n);
    vector<vector<int>> g(x+1);
    vector<int> ans;
    vector<bool> seen(x+1,false);
    seen[x] = true;
    int min_val = 1<<30;
    queue<int> que;

    rep(i,n) {
        int k = max(min_val, int(a[i]));
        rep2(j, k, x+1) {
            g[j].emplace_back(j%k);
            if(seen[j]&&!seen[j%k]) {
                seen[j%k] = true;
                que.push(j%k);
            }
        }
        while(!que.empty()) {
            int p = que.front(); que.pop();
            for(auto ni: g[p]) {
                if(!seen[ni]) {
                    seen[ni] = true;
                    que.push(ni);
                }
            }
        }

        chmin(min_val, int(a[i]));
        if(min_val <= y) break;

        rep(j,x+1) {
            int p = min_val*j+y;
            if(p > x) break;
            if(seen[p]) {
                ans.emplace_back(i+1);
                break;
            }
        }
    }
    
    cout << ans.size() << "\n";
    rep(i,ans.size()) cout << ans[i] << ((i+1==n)?"":" ");
    cout << "\n";
}

int main() {
    cin.tie(0);
    ios::sync_with_stdio(false);
    int t=1;
    cin >> t;
    while(t--)solve();
}


