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

int ans = 0;
int a[200000+2];
int state[200000+2];

void dfs(int i, int n, int cnt, int mx, int re) {
    if(i==n) {
        ans = min(ans, mx);
        return;
    }
    if(ans<=mx) {
        return;
    }

    if(state[a[i]] == 1) {
        // 選ぶ(強制)
        dfs(i+1, n, cnt, max(mx, re), 0);
    } else if(state[a[i]] == 2) {
        // 選ばない
        dfs(i+1, n, cnt, max(mx, re+1), re+1);
    } else {
        if(cnt < 2) {
            state[a[i]] = 1;
            dfs(i+1, n, cnt+1, max(mx, re), 0);
            state[a[i]] = 0;
        } 
        state[a[i]] = 2;
        dfs(i+1, n, cnt, max(mx, re+1), re+1);
        state[a[i]] = 0;
    }
}

void solve() {
    int n, k; cin >> n >> k;
    rep(i, n) {
        cin >> a[i];
        a[i]--;
    }
    rep(i, n) state[i] = 0;
    ans = n;
    dfs(0, n, 0, 0, 0);
    cout << ans << '\n';
}

int main() {
    cin.tie(0);
    ios::sync_with_stdio(false);

    int t;
    cin >> t;
    while(t--) solve();
}
