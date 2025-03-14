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
    int n, m; cin >> n >> m;

    vp e(m);
    ll u, v;
    rep(i, m) {
        cin >> u >> v;
        u--; v--;
        e[i] = {u, v};
    }

    if(m%2==1) {
        cout << "-1\n";
        return;
    }
    vvll g(n);

    for(auto[u, v]: e) {
        if(u==0&&v==n-1) {
            cout << "-1\n";
            return;
        }
        g[u].push_back(v);
        g[v].push_back(u);
    }

    vector<bool> seen(n, false);
    seen[0] = true;
    stack<ll> stk;
    stk.push(0);
    while(!stk.empty()) {
        ll p = stk.top(); stk.pop();
        if(p>=0) {
            for(ll ni: g[p]) {
                if(!seen[ni]) {
                    seen[ni] = true;
                    if(ni != n-1) {
                        stk.push(ni);
                    }
                }
            }
        }
    }

    vector<bool> itrvl(n, false);
    itrvl[n-1] = true;
    stk.push(n-1);
    while(!stk.empty()) {
        ll p = stk.top(); stk.pop();
        for(ll ni: g[p]) {
            if(seen[ni] && !itrvl[ni]) {
                if(ni != 0) {
                    stk.push(ni);
                    itrvl[ni] = true;
                }
            }
        }
    }


    vector<char> ans(m, '@');
    ll cnt_b = 0;
    ll cnt_r = 0;
    rep(i, m) {
        ll u = e[i].first;
        ll v = e[i].second;
        if(u==0 && itrvl[v]) {
            cnt_b += 1;
            ans[i] = 'B';
        } else if(itrvl[u] && v == n-1) {
            cnt_r += 1;
            ans[i] = 'R';
        } 
    }

    if(cnt_b*2>m || cnt_r*2>m) {
        cout << "-1\n";
        return;
    } else {
        rep(i, m) {
            if(ans[i] == '@') {
                if(cnt_b*2 < m) {
                    ans[i] ='B';
                    cnt_b += 1;
                }
                else ans[i] = 'R';
            }
        }
        ll c1 = 0, c2 = 0;
        rep(i, m) {
            if(ans[i] == 'R') c1 ++;
            else c2 ++;
            cout << ans[i];
        }
        cout << '\n';
    }
}

int main() {
    cin.tie(0);
    ios::sync_with_stdio(false);
    int t=1;
    //cin >> t;
    while(t--)solve();
}
