#include <bits/stdc++.h>
#include "input.hpp"

using namespace std;

// using namespace atcoder;

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
    int h, w; cin >> h >> w;
    string s; cin >> s;
    ll d_cnt = 0, r_cnt = 0;
    for(char c: s) if(c=='D')d_cnt++;
                   else if(c=='R') r_cnt++;
    
    vp trace_d_first, trace_r_first;

    // d優先の場合
    {
        P cur = {0,0};
        trace_d_first.emplace_back(cur);
        ll used_d = d_cnt;
        rep(i,s.size()) {
            if(s[i]=='D') {
                cur.first ++;
            } else if(s[i]=='R'){
                cur.second ++;
            } else {
                if(used_d < h-1){
                    used_d++;
                    cur.first ++;
                } else {
                    cur.second++;
                }
            }
            trace_d_first.emplace_back(cur);
        }
    }

    // r優先
    {
        P cur = {0,0};
        trace_r_first.emplace_back(cur);
        ll used_r = r_cnt;
        rep(i,s.size()){
            if(s[i]=='D'){
                cur.first++;
            } else if(s[i]=='R'){
                cur.second++;
            } else {
                if(used_r < w-1){
                    used_r++;
                    cur.second++;
                } else {
                    cur.first++;
                }
            }
            trace_r_first.emplace_back(cur);
        }
    }

    vll up(w, inf), lwr(w, -1);
    rep(i, trace_d_first.size()){
        auto [ri, rj] = trace_r_first[i];
        chmin(up[rj], ri);

        auto [di, dj] = trace_d_first[i];
        chmax(lwr[dj], di);
    }

    ll ans = 0;
    rep(i,w){
        ans += lwr[i] - up[i] + 1;
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


