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


int solve(){
    int n, d; cin >> n >> d;
    if(n==0) return 1;
    vp v(n); rep(i,n) cin >> v[i].first >> v[i].second;
    if(n==1) v.push_back({-inf,0});

    sort(all(v), [](const P& a, const P& b) {
        if(a.first==b.first) return a.second < b.second;
        else return a.first > b.first;
    });

    deque<ll> dq;
    ll ans = 0;
    rep(i,d) {
        dq.push_back(v[0].first - v[0].second * i);
        ans += v[0].first - v[0].second * i;
    }

    for(int i=1; i<d; i+=2) {
        ll bc = dq.back(); // 減少量
        dq.pop_back();
        ll inc = v[1].first + (d-i-1) * v[0].second;
        if(bc<inc) {
            ans = ans - bc + inc;
            dq.pop_back();
        }
    }
    
    cout << ans << '\n';
    return 0;
}

int main() {
    cin.tie(0);
    ios::sync_with_stdio(false);
    while(!solve());
}


