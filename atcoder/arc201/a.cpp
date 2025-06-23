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
    int n; cin >> n;
    vector<tuple<ll,ll,ll>> v(n);
    ll base = 0;
    
    for(auto& [x,y,z]: v) {
        cin >> x >> y >> z;

        ll val = min(x,max(ll(0),y-z));
        base += val;
        x -= val;
        y -= val;
    }

    auto check = [&](ll key) -> bool {
        ll div1 = base;
        ll div2 = 0;
        for(auto [x,y,z]: v) {
            if(div1>=key) {
                div2 += min(y,z);
            } else {
                ll left = key-div1;

                ll val = min(min(x,y), left);
                div1 += val;
                div2 += min(y-val, z);
            }
        }
        return div1>=key && div2>=key;
    };

    ll ok = 0;
    ll ng = 1e18;
    while(abs(ng-ok)>1) {
        ll mid = ok + (ng-ok)/2;
        if(check(mid)) {
            ok = mid;
        } else {
            ng = mid;
        }
    }
    cout << ok << '\n';

}

int main() {
    cin.tie(0);
    ios::sync_with_stdio(false);
    int t=1;
    cin >> t;
    while(t--)solve();
}


