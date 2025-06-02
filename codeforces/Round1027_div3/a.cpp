#line 1 "a.cpp"
#include <bits/stdc++.h>
#line 1 "/home/ardririy/repos/cp-libs/libraries-cpp/input.hpp"


#line 6 "/home/ardririy/repos/cp-libs/libraries-cpp/input.hpp"
using namespace std;
vector<long long> i64_vec_IN(int n) {vector<long long> res(n); for(int i=0; i<n; i++) { cin >> res[i]; } return res; }
vector<string> str_vec_IN(int n) { vector<string> res(n); for(int i=0; i<n; i++) { cin >> res[i]; } return res; }
vector<vector<int>> graph_IN(int n, int m) { vector<vector<int>> g(n); int u, v; for(int i=0; i<m; i++) { cin >> u >> v; u--; v--; g[u].emplace_back(v); g[v].emplace_back(u); } return g; }
vector<vector<pair<int, long long>>> weighted_graph_IN(int n, int m) { vector<vector<pair<int, long long>>> g(n); int u, v; long long w; for(int i=0; i<m; i++) { cin >> u >> v >> w; u--; v--; g[u].push_back({v, w}); g[v].push_back({u, w}); } return g; }


#line 3 "a.cpp"

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
    string s; cin >> s;
    ll ns = stoi(s);
    rep(i,101) {
        ll k = i*i;
        if(k==ns) {
            cout << 0 << " " << i << '\n';
            return;
        }
    }
    cout << "-1\n";
}

int main() {
    cin.tie(0);
    ios::sync_with_stdio(false);
    int t=1;
    cin >> t;
    while(t--)solve();
}


