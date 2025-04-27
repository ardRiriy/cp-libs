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

struct comp {
    bool operator()(const tuple<ll,ll,string>& a, tuple<ll,ll,string>& b) {
        return get<2>(a).size() > get<2>(b).size();
    }
};

void solve() {
    int n; cin >> n;

    // 0: 掛け算/数 1: 足し算
    vector<vector<string>> dist(n+1, vector<string>(2, ""));

    // queue: pair<ll, pair<ll, pair<ll, ll>>> {コスト, 0/1, str}
    priority_queue<tuple<ll,ll,string>, vector<tuple<ll,ll,string>>, comp> pq;

    ll snum = 1;
    string s = "1";
    while(snum<=n) {
        pq.push({snum, 0, s});
        s += '1';
        snum *= 10;
        snum += 1;
    }

    while(!pq.empty()) {
        auto [val, typ, str] = pq.top(); pq.pop();
        if(!dist[val][typ].empty())continue;
        dist[val][typ] = str;

        if(typ==0) {
            rep2(i, 1, n+1) {
                // 掛け算/数
                ll nxt = i*val;
                if(nxt<=n) {
                    if(!dist[i][0].empty()) {
                        string nstr = str + "*" + dist[i][0];
                        pq.push({nxt, 0, nstr});
                    }
    
                    if(!dist[i][1].empty()) {
                        string nstr = str + "*(" + dist[i][1] + ")";
                        pq.push({nxt, 0, nstr});
                    }
                }

                nxt = i+val;
                if(nxt<=n) {
                    if(!dist[i][0].empty()) {
                        string nstr = str + "+" + dist[i][0];
                        pq.push({nxt, 1, nstr});
                    }
                    if(!dist[i][1].empty()) {
                        string nstr = str + "+" + dist[i][1];
                        pq.push({nxt, 1, nstr});
                    }
                }
            }
        } else {
            rep2(i, 1, n+1) {
                // 掛け算/数
                ll nxt = i*val;
                if(nxt<=n) {
                    if(!dist[i][0].empty()) {
                        string nstr = "(" + str + ")*" + dist[i][0];
                        pq.push({nxt, 0, nstr});
                    }
    
                    if(!dist[i][1].empty()) {
                        string nstr = "(" + str + ")*(" + dist[i][1] + ")";
                        pq.push({nxt, 0, nstr});
                    }
                }

                nxt = i+val;
                if(nxt<=n) {
                    if(!dist[i][0].empty()) {
                        string nstr = str + "+" + dist[i][0];
                        pq.push({nxt, 1, nstr});
                    }
                    if(!dist[i][1].empty()) {
                        string nstr = str + "+" + dist[i][1];
                        pq.push({nxt, 1, nstr});
                    }
                }
            }
        }
    }
    if(dist[n][0].empty()) {
        cout << dist[n][1] << '\n';
    } else if(dist[n][1].empty()) {
        cout << dist[n][0] << '\n';
    } else if(dist[n][0].size() < dist[n][1].size()) {
        cout << dist[n][0] << '\n';
    } else {
        cout << dist[n][1] << '\n';
    }
}

int main() {
    cin.tie(0);
    ios::sync_with_stdio(false);
    int t=1;
    // cin >> t;
    while(t--)solve();
}

