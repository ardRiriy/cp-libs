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

ll di[] = {1, 0, -1, 0, -1, 1, -1, 1};
ll dj[] = {0, 1, 0, -1, -1, 1, 1, -1};

void solve() {
    ll n; cin >> n;
    vvll a(n, vll(n, inf));

    int ci = 0, cj = 0;
    ll cur = n*n-1;
    ll r = 0;
    rep(_,n) rep(__,n) {
        a[ci][cj] = cur--;

        while(true) {
            int ni = ci+di[r];
            int nj = cj+dj[r];
            if(ni>=0&&nj>=0&&ni<n&&nj<n&&a[ni][nj]==inf) {
                ci = ni;
                cj = nj;
                break;
            } else {
                r = (r+1)%4;
                if(cur==-1) break;
            }
        }
    }
    rep(i,n) rep(j,n) cout << a[i][j] << ((j+1==n)?'\n':' ');
}

int main() {
    cin.tie(0);
    ios::sync_with_stdio(false);
    int t=1;
    cin >> t;
    while(t--)solve();
}


