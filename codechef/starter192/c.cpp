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
    auto a = i64_vec_IN(n);
    auto g = graph_IN(n,n-1);

    map<int,int> mp;
    rep(i,n) {
        if(a[i] != 0) mp[a[i]] = i;
    }

    set<int> exclude;

    vector<int> cnt(n,0);
    rep(i,n) {
        if(a[i] > 0){
            for(auto ni: g[i]) {
                cnt[ni]++;
            }
        }
        
    }
    dbg(mp);
    djks pq;
    int left = 0;
    rep(i,n) {
        if(a[i]==0) pq.push({g[i].size()-cnt[i],i}), left++;
    }

    ll ans = n+1;
    per(i,n) {
        dbg(i);
        // i+1を達成可能か？
        if(!mp.contains(i+1)) {
            left--;

            if(pq.empty()) continue;
            auto [cnt, j] = pq.top();
            pq.pop();

            while(exclude.contains(j) && !pq.empty()) {
                auto [c, ii] = pq.top();
                pq.pop();
                cnt = c;
                j = ii;
            }
            pq.push({cnt,j});
            if(exclude.contains(j)) {
                continue;
            }
            if(left>=cnt) {
                dbg(i);
                ans=i+1;
            }
        } else {
            auto p = mp[i+1];
            dbg(p);
            if(!exclude.contains(p) && left>=g[p].size()-cnt[p]) {
                ans=i+1;
            }
            for(auto ni: g[p]) {
                exclude.insert(ni);
            }
        }
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


