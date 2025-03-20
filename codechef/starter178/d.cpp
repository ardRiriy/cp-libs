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
    vll a(n); rep(i,n) cin>>a[i];
    if(n>40) {
        rep(i, n) cout << 0;
        cout << '\n';
        return;
    }

    vector<int> ans(n,0);
    rep(i, n) {

        int cur = 0;
        vvll v(2, vll());

        // 初期
        rep(j, n) {
            if(i==j) continue;
            v[cur].push_back(a[j]);
        }

        bool flag = true;
        while(v[cur].size() > 1) {
            int nxt = 1-cur;
            rep(j, v[cur].size()-1) {
                if(v[cur][j+1] - v[cur][j] <= 0) flag = false;
                v[nxt].push_back(v[cur][j+1] - v[cur][j]);
            }
            v[cur].clear();
            cur=nxt;
        }
        if(flag) ans[i] = 1;
    }
    rep(i,n)cout<<ans[i];
    cout<<'\n';
}

int main() {
    cin.tie(0);
    ios::sync_with_stdio(false);
    int t=1;
    cin >> t;
    while(t--)solve();
}


