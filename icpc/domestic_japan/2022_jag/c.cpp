#include <bits/stdc++.h>
#include "input.hpp"

#include "atcoder/modint.hpp"

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

#define rep(i,n) for (ll i=0;i<(n);++i)
#define per(i,n) for(ll i=(n)-1;i>=0;--i)
#define rep2(i,a,n) for (ll i=(a);i<(n);++i)
#define per2(i,a,n) for (ll i=(n)-1;i>=(a);--i)


template<class T>bool chmax(T &a, const T &b) { if (a<b) { a=b; return true; } return false; }
template<class T>bool chmin(T &a, const T &b) { if (b<a) { a=b; return true; } return false; }

ll dx[] = {1, 0, -1, 0, -1, 1, -1, 1};
ll dy[] = {0, 1, 0, -1, -1, 1, 1, -1};

void solve(ll n, ll m) {
    if(n==0) {
        cout << -m*m << '\n';
        return;
    } else if(m==0) {
        cout << n*n << '\n';
        return;
    } else if(m==1) {
        cout << n*n-1 << '\n';
        return;
    }

    ll ans = -inf;
    ll x = 1;
    while(true) {
        if(n<x || m < x+1) break;
        ll n_p = x-1;
        n_p += (n-(x-1))*(n-(x-1));

        ll p = m / (x+1);
        ll q = m % (x+1);
        ll m_p = q*(p+1)*(p+1) + (x+1-q)*p*p;
        chmax(ans,n_p-m_p);
        x+=1;
    }
    cout << ans << '\n';
}

int main() {
    cin.tie(0);
    ios::sync_with_stdio(false);
    int n, m;
    cin >> n >> m;
    while(n>0||m>0) {
        solve(n, m);
        cin >> n >> m;
    }
}
