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
    vector<bool> isPrime(n+1, true);
    isPrime[0] = false;
    isPrime[1] = false;

    rep(i,n+1) {
        if(!isPrime[i]) continue;
        
        ll cur = 2*i;
        while(cur<n+1) {
            isPrime[cur] = false;
            cur += i;
        }
    }
    vll res;
    rep(i,n+1) if(isPrime[i]) res.emplace_back(i);
    return res;
}

void solve() {
    ll l, r; cin >> l >> r;    

    vll p = primes(sqrt(r)+10);

    // l+i が素数べきであるか？
    vector<bool> isPrimePower(r-l+1, true);
    for(auto pi: p) {
        ll cur = pi * (l/pi + ((l%pi==0) ? 0 : 1));
        while(cur<=r) {
            isPrimePower[cur-l] = false;
            cur+=pi;
        }
    }

    // 素数べきが弾かれているのでこれを戻す
    for(auto pi: p) {
        ll cur = pi;
        while(cur<=r) {
            if(cur>=l) isPrimePower[cur-l] = true;
            cur*=pi;
        }
    }

    ll ans=0;
    for(auto t: isPrimePower) if(t) ans++;
    if(isPrimePower[0]) ans--;
    cout << ans + 1 << '\n';
}

int main() {
    cin.tie(0);
    ios::sync_with_stdio(false);
    int t=1;
    //cin >> t;
    while(t--)solve();
}


