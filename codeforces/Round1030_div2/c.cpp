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
    ll n, k; cin >> n >> k;
    auto a = i64_vec_IN(n);
    ll current = 0;
    djks pq;
    for(auto ai: a) {
        current += __popcount(ai);
        ll cost = 0;
        ll base = 0;
        rep(i,63) if(((ai>>i)&1) != 1) {
            cost = (1ll << (i+1)) - 1ll - base;
            break;
        } else {
            base = base | (1ll << i);
        }
        pq.push({cost, ai});
    }

    ll additional = 0;
    while(pq.top().first<=k) {
        auto [c, v] = pq.top();
        pq.pop();
        k -= c;
        v += c;
        additional += 1;
        ll nc = 0;
        ll base = 0;
        rep(i,63) 
            if(((v>>i)&1) != 1) {
            nc = (1ll << (i+1)) - 1ll - base;
            break;
        } else {
            base = base | (1ll << i);
        }
        pq.push({nc, v});
    }
    cout << current + additional << '\n';
}

int main() {
    cin.tie(0);
    ios::sync_with_stdio(false);
    int t=1;
    cin >> t;
    while(t--)solve();
}
