#include <bits/stdc++.h>
#include "input.hpp"

using namespace std;
// using namespace atcoder;

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
const double PI = acos(-1);

#define rep(i,n) for (ll i=0;i<(n);++i)
#define per(i,n) for(ll i=(n)-1;i>=0;--i)
#define rep2(i,a,n) for (ll i=(a);i<(n);++i)
#define per2(i,a,n) for (ll i=(n)-1;i>=(a);--i)

constexpr ll m = 1e9+7;
template<class T>bool chmax(T &a, const T &b) { if (a<b) { a=b; return true; } return false; }
template<class T>bool chmin(T &a, const T &b) { if (b<a) { a=b; return true; } return false; }

ll dx[] = {1, 0, -1, 0, -1, 1, -1, 1};
ll dy[] = {0, 1, 0, -1, -1, 1, 1, -1};

vvll matrix_multiplication(vvll& a, vvll& b, ll mod) {
    ll n = a.size();
    vvll res(n, vll(n, 0));
    rep(i,n) {
        rep(j,n) {
            rep(k,n) {
                res[i][k] += a[i][j] * b[j][k];
                res[i][k] %= mod;
            }
        }
    }
    return res;
}

void solve() {
    ll n, k; cin >> n >> k;
    vvll a(n, vll(n)); rep(i,n)rep(j,n)cin>>a[i][j];
    vector<vvll> v;
    v.push_back(a);

    ll N = 63;
    rep(i,N) {
        v.push_back(matrix_multiplication(v[i], v[i], m));
    }

    vvll ans(n, vll(n, 0));
    rep(i,n) ans[i][i] = 1;
    rep(i,N) {
        if(((k>>i)&1)==1) {
            ans = matrix_multiplication(ans, v[i], m);
        } 
    }
    ll sum = 0;
    rep(i,n) rep(j,n) sum = (sum + ans[i][j]) % m;
    cout << sum << '\n';
}


int main() {
    cin.tie(0);
    ios::sync_with_stdio(false);
    int t=1;
    //cin >> t;
    while(t--)solve();
}


