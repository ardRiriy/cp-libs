#include <bits/stdc++.h>
#include <atcoder/all>

using namespace std;
using namespace atcoder;

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
    vector<string> s(n);
    rep(i, n) cin >> s[i];
    vvll g(n), rg(n);
    rep(i,n) {
        rep(j,m) {
            if(s[i][j] == '1') {
                g[i].push_back(i+j+1);
                rg[i+j+1].push_back(i);
            }
        }
    }

    vll dist(n, inf);
    dist[0] = 0;
    queue<ll> que;
    que.push(0);
    while(!que.empty()) {
        ll v = que.front(); que.pop();
        for(auto nv : g[v]) {
            if(dist[nv]==inf) {
                dist[nv] = dist[v] + 1;
                que.push(nv);
            }
        }
    }

    vll rev_dist(n, inf);
    rev_dist[n-1] = 0;
    assert(que.empty());
    que.push(n-1);

    while(!que.empty()) {
        ll v = que.front(); que.pop();
        for(auto nv : rg[v]) {
            if(rev_dist[nv]==inf) {
                rev_dist[nv] = rev_dist[v] + 1;
                que.push(nv);
            }
        }
    }

    vll ans(n, inf);
    rep2(i, 1, n-1) {
        // 前後Mを見る
        rep(j, m+1) {
            if(i-j-1 < 0) break;
            for(auto nv : g[i-j-1]) {
                if(nv<=i) continue;
                chmin(ans[i], dist[i-j-1] + rev_dist[nv] + 1);
                
            }
        }
    }

    rep2(i, 1, n-1) {
        cout << (ans[i] == inf ? -1 : ans[i]) << (i == n-2 ? "\n" : " ");
    }
}

int main() {
    cin.tie(0);
    ios::sync_with_stdio(false);
    int t=1;
    //cin >> t;
    while(t--)solve();
}


