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

void solve(int n) {
    vvll a(n, vll(n));
    rep(i,n) rep(j,n) cin >> a[i][j];

    int m = n/2;
    vvll b = a;
    rep(i,n) {
        int base = (i%2)?0:m;
        rep(j,n) {
            b[i][(j+base)%n] = a[i][j];
        }
    }

    vvll c = a;
    rep(j,n) {
        int base = (j%2)?0:m;
        rep(i,n){
            c[(i+base)%n][j] = b[i][j];
        }
    }

    rep(i,n) {
        rep(j,n) {
            cout << c[i][j] << ((j+1==n)?'\n':' ');
        }
    }
}

int main() {
    cin.tie(0);
    ios::sync_with_stdio(false);
    int n; cin >> n;    
    while(n){
        solve(n);
        cin >> n;
    };
}