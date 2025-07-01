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

vll primes(ll n) {
    vector<bool> c(n+1, true);
    c[0] = false;
    c[1] = false;
    rep2(i,2,n+1) {
        if(!c[i]) continue;
        ll cur = 2*i;
        while(cur<=n) {
            c[cur] = false;
            cur += i;
        }
    }
    vll res;
    rep(i,n+1) if(c[i]) res.emplace_back(i);
    return res;
}

void solve() {
    int n; cin >> n;
    vll p = primes(n);

    vll ans(n,-1);
    ans[0] = 1;
    per(i,p.size()) {
        vll ns;
        int buf = -1;
        ll cur = p[i];
        while(cur<=n) {
            if(ans[cur-1]==-1) {
                ns.emplace_back(cur);
            }
            cur += p[i];
        }
        rep(j,ns.size()) {
            ans[ns[(j+1)%ns.size()]-1] = ns[j];
        }
    }

    rep(i, n) cout << ans[i] << " \n"[i+1==n];
    #ifdef ADRY
        rep2(i,1,n) assert(gcd(ans[i],i+1)>1);
        int cnt = 0;
        rep(i,n) if(i+1 != ans[i]) {cnt++; dbg(i+1); };
        dbg(cnt);
    #endif
    

}

void checker() {
    int a[] = {1,12,9,6,10,8,7,4,3,5,11,2,13};
    int cnt = 0;
    rep(i,13) if(i+1 != a[i]) {cnt++; dbg(i+1); }
    dbg(cnt);
}

int main() {
    cin.tie(0);
    ios::sync_with_stdio(false);
    int t=1;
    cin >> t;
    while(t--)solve();
}


