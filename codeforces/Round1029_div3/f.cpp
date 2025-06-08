#include <bits/stdc++.h>
#include "input.hpp"
#include "atcoder/modint.hpp"
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

using mint = modint1000000007;

void solve() {
    int n; cin >> n;
    auto g = graph_IN(n,n-1);

    {
        int cnt = 0;
        bool isCreatable = true;

        rep(i,n) {
            if(g[i].size()>3 || (i==0 && g[i].size()>2)) isCreatable = false;
            else if(g[i].size()==3 || (i == 0 && g[i].size()==2)) cnt++;
        }

        if(!isCreatable || cnt>1) {
            cout << "0\n";
            return;
        }
        if(cnt==0) {
            // path
            mint ans = mint(2).pow(n);
            cout << ans.val() << '\n';
            return;
        }
    }

    ll m = 0;
    ll cur = 0;
    vector<bool> seen(n, false);
    while(true) {
        m++;
        seen[cur] = true;
        if(g[cur].size()==3 || (cur==0 && g[cur].size()==2)) break;
        for(auto ni: g[cur]) {
            if(!seen[ni]) cur = ni;
        }
    }
    ll l = -1;
    ll r = -1;
    rep(i,(cur==0)?2:3) {
        auto ni = g[cur][i];
        if(seen[ni]) continue;
        ll k = ni;
        ll cnt = 0;
        while(!seen[k]) {
            cnt++;
            seen[k] = true;
            for(auto nni: g[k]) {
                if(!seen[nni]) k = nni;
            }
        }
        if(l==-1) l = cnt;
        else r = cnt;
    }

    dbg(l,r,m);
    mint ans = mint(2).pow(m) * (mint(2).pow(max(abs(l-r)-1, 0ll)) + mint(2).pow(abs(l-r)));
    cout << ans.val() << '\n';

}

int main() {
    cin.tie(0);
    ios::sync_with_stdio(false);
    int t=1;
    cin >> t;
    while(t--)solve();
}


