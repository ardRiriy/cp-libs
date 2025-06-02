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
    int n, m, l; cin >> n >> m >> l;
    auto a = i64_vec_IN(l);

    ll kisuu_min = inf;
    ll guusuu_min = inf;
    ll sum = 0;
    for(auto ai: a) {
        sum += ai;
        if(ai%2==1) chmin(kisuu_min, ai);
        else chmin(guusuu_min, ai);
    }

    ll kisuu_max = -1;
    ll guusuu_max = -1;
    if(sum%2==0) {
        guusuu_max = sum;
        if(kisuu_min != inf) kisuu_max = sum - kisuu_min;
    } else {
        kisuu_max = sum;
        if(kisuu_min != inf) guusuu_max = sum - kisuu_min;
    }

    auto g = graph_IN(n, m);

    vector<vector<ll>> dist(n, vll(2, inf));
    dist[0][0] = 0;

    queue<pair<int,int>> que;
    que.push({0,0});

    while(!que.empty()) {
        auto [p, digi] = que.front();
        que.pop();

        ll next_dist = dist[p][digi]+1;
        ll next_digi = next_dist % 2;
        for(auto ni: g[p]) {
            if(dist[ni][next_digi] == inf) {
                dist[ni][next_digi] = next_dist;
                que.push({ni, next_digi});
            }
        }
    }

    dbg(dist);
    dbg(kisuu_max);
    dbg(guusuu_max);
    string ans = "";
    rep(i,n) {
        if(i==0) {
            ans += '1';
            continue;
        }

        if(dist[i][0] != inf && dist[i][0] <= guusuu_max) {
            ans += '1';
            continue;
        }
        if(dist[i][1] != inf && dist[i][1] <= kisuu_max) {
            ans += '1';
        } else {
            ans += '0';
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


