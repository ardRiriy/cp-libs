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


int solve(){
    ll n, m; cin >> n >> m;
    auto s = i64_vec_IN(n);
    if(n==0) return 1;

    auto check = [&](ll key) -> bool {
        // keyのときに交戦回数がM以上になるか
        ll cnt = 0;
        ll level = 1;
        while(true) {
            auto itr = lower_bound(all(s), level + key);
            if(itr == s.begin()) {
                return true; // 無限回の交戦とみなす
            }
            cnt++;
            ll val = max(1ll,key-abs(level-*(itr-1)));
            level += val;
            if(itr-s.begin() == n-1) break;
        }
        return cnt>=m;
    };

    ll ok = -1;
    ll ng = s[n-1]+1;
    while(abs(ok-ng)>1) {
        dbg(ok,ng);
        ll mid = (ok+ng)>>1;
        if(check(mid)) ok = mid;
        else ng = mid;
    }

    if(ok==-1 || 1+ok>=s[n-1]) {
        cout << "-1\n";
    } else {
        cout << ok << '\n';
    }
    return 0;
}

int main() {
    cin.tie(0);
    ios::sync_with_stdio(false);
    while(!solve());
}


