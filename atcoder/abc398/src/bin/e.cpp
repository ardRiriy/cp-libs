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

void solve() {
    int n; cin >> n;
    int u, v;
    vector<pair<int, int>> vec(n-1);
    vector<vector<ll>> g(n);
    rep(i, n-1) {
        cin >> u >> v;
        u--; v--;
        vec[i] = {u, v};
        g[u].push_back(v);
        g[v].push_back(u);
    }

    vll color(n, -1);
    
    color[0] = 0;
    stack<ll> stk;
    stk.push(0);

    while(!stk.empty()) {
        int p = stk.top(); stk.pop();
        for(ll ni: g[p]) {
            if(color[ni] != -1) continue;
            color[ni] = 1-color[p];
            stk.push(ni);
        }
    }

    int cnt[] = {0, 0};
    for(int i: color) cnt[i]++;

    ll ans = cnt[0]*cnt[1] - (n-1);
    set<pair<int,int>> st;
    rep(i,n)rep2(j,i+1,n)if(color[i]!=color[j])st.insert({i,j});
    for(auto[u, v]: vec) st.erase({u,v});
    
    if(ans%2==0) {
        cout << "Second" << endl;
        int i, j; cin >> i >> j;
        i--; j--;
        if(i==-1&&j==-1){
            return;
        }
        st.erase({i,j});
    } else {
        cout << "First" << endl;
    }

    int i=0,j=0;
    while(!(i==-2&&j==-2)){
        auto [a, b] = *st.begin();
        st.erase({a,b});
        cout << a+1 << ' ' << b+1 << endl;
        cin >> i >> j;
        i--; j--;
        st.erase({i,j});
    }
}

int main() {
    int t=1;
    //cin >> t;
    while(t--)solve();
}


