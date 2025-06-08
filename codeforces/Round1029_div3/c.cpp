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
    int n; cin >> n;
    auto a = i64_vec_IN(n);
    for(ll& ai: a) ai--;
    dbg(a);
    
    vvll g(n);
    rep(i,n) g[a[i]].emplace_back(i);
    ll ans = 1;
    ll l = 0;
    ll r = 1;
    while(true) {
        dbg(l,r);
        // [l, r)で次の一番右を見つける
        bool isEndLoop = false;
        ll next_r = -1;
        rep2(i,l,r) {
            auto itr = upper_bound(all(g[a[i]]),i);
            if(itr == g[a[i]].end()) {
                isEndLoop = true; 
                break;
            }
            chmax(next_r, *itr);
        }
        if(isEndLoop) break;
        ans++;

        rep2(i,r,next_r) {
            auto itr = upper_bound(all(g[a[i]]), i);
            if(itr == g[a[i]].end() || *itr >= next_r) break;
            r++;
        }
        l = r;
        r = next_r+1;
    }
    cout << ans << '\n';
}

int main() {
    cin.tie(0);
    ios::sync_with_stdio(false);
    int t=1;
    cin >> t;
    while(t--)solve();
}


 