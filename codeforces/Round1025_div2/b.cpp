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

void solve() {
    int n, m, a, b; cin >> n >> m >> a >> b;

    // 初手: 短い方を選べる
    int n_d = min(n-a+1, a);
    int m_d = min(m-b+1, b);

    ll ans = 0;

    int n1 = 0;
    while(n>1) {
        n1++;
        n = n / 2 + ((n%2==0) ? 0 : 1);
    }
    int n2 = 1;
    while(n_d>1) {
        n2++;
        n_d = n_d / 2 + ((n_d%2==0) ? 0 : 1);
    }

    int m1 = 0;
    while(m>1) {
        m1++;
        m = m / 2 + ((m%2==0) ? 0 : 1);
    }
    int m2 = 1;
    while(m_d>1) {
        m2++;
        m_d = m_d / 2 + ((m_d%2==0) ? 0 : 1);
    }

    cout << min(n1+m2, n2+m1) << '\n';
}

int main() {
    cin.tie(0);
    ios::sync_with_stdio(false);
    int t=1;
    cin >> t;
    while(t--)solve();
}


