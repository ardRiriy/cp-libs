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
    int n, q; cin >> n >> q;
    string s1, s2; cin >> s1 >> s2;
    reverse(all(s2));

    vector<int> p1, p2;
    rep(i,n) if(s1[i]=='1') p1.emplace_back(i);
    rep(i,n) if(s2[i]=='1') p2.emplace_back(i);

    // 最後に使ったindex
    int lu1 = -1, lu2 = -1;
    ll c1 = 0, c2 = 0;
    vll costs1, costs2;

    rep(i,n) {
        auto itr = upper_bound(all(p1), lu1);
        if(itr==p1.end()) {
            // 構成不可能
            costs1.emplace_back(-1);
        } else {
            int j = *itr;
            lu1 = j;
        
            c1 += j-i;
            costs1.emplace_back(c1);
        }

        itr = upper_bound(all(p2), lu2);
        if(itr==p2.end()) {
            costs2.emplace_back(-1);
        } else {
            int j = *itr;
            lu2 = j;
            c2 += j-i;
            costs2.emplace_back(c2);
        }
    }


    ll ans = inf;
    reverse(all(costs2));
    rep(i,n) {
        if(costs1[i]==-1 || costs2[i]==-1) continue;
        chmin(ans, costs1[i] + costs2[i]);
    }
    cout << ((ans==inf)?-1:ans) << '\n';

    dbg(costs1);
    dbg(costs2);
}

int main() {
    cin.tie(0);
    ios::sync_with_stdio(false);
    int t=1;
    cin >> t;
    while(t--)solve();
}


