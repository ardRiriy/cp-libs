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

ll dx[] = {1, 0, -1, 0, -1, 1, -1, 1};
ll dy[] = {0, 1, 0, -1, -1, 1, 1, -1};

ll floor_sum(ll n, ll m, ll a, ll b) {
    ll a1 = a/m;
    ll a2 = a%m;
    ll b1 = b/m;
    ll b2 = b%m;
    ll s = n * (n - 1) / 2 * a1;
    if (a2==0) {
        return s + b1 * n;
    }
    ll k = (a2 * (n-1) + b2) / m;
    return s + n * (k + b1) - floor_sum(k, a2, m, m + a2 - b2 - 1);
}

void solve() {
    ll a, b, c, d; cin >> a >> b >> c >> d;
    ll ans = floor_sum((d-2)/(c-b), d, c, a+c) - floor_sum((d-2)/(c-b),d,b,a+b-1);
    cout << (d-2)/(c-b) - ans << '\n';
}

int main() {
    cin.tie(0);
    ios::sync_with_stdio(false);
    int t=1;
    cin >> t;
    while(t--)solve();
}


