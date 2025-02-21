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

void solve() {
    int n, m, k;
    cin >> n >> m >> k;
    vvll edges(m);
    rep(i, m) {
        int a, b, c;
        cin >> a >> b >> c;
        a--; b--;
        edges[i] = {a, b, c};
    }

    vll dist(n, inf);
    dist[0] = 0; 
    rep(i, k) {
        int e; cin >> e; e--;
        if (dist[edges[e][0]] != inf) {
            chmin(dist[edges[e][1]], dist[edges[e][0]] + edges[e][2]);
        }
    }
    cout << ((dist[n-1] == inf) ? -1 : dist[n-1]) << endl;
}

int main() {
    cin.tie(0);
    ios::sync_with_stdio(false);
    solve();
}