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

const int inf=1ll<<30;
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

vll divisors(ll n) {
    vll res;
    rep2(i,1, sqrt(n)+10) {
        if(n%i==0) {
            res.emplace_back(i);
            if(i!=n/i) res.emplace_back(n/i);
        }
    }

    return res;
}

int solve(){
    ll b;
    cin >> b;

    if(b==0) return 1;
    vll div = divisors(2*b);
    P ans = {-1,-1};
    for(auto di: div) {
        ll t = 2*b/di;
        ll rhs = t+1-di;
        if(rhs<=0 || rhs%2==1) continue;
        if(chmax(ans.second, di)) {
            ans.first = rhs/2;
        }
    }
    cout << ans.first << " " << ans.second << '\n';
    return 0;
}

int main() {
    cin.tie(0);
    ios::sync_with_stdio(false);
    while(!solve());
}


