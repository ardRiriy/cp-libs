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

ll di[] = {1, 0, -1, 0, -1, 1, -1, 1};
ll dj[] = {0, 1, 0, -1, -1, 1, 1, -1};

void solve() {
    int h, w; cin >> h >> w;
    vector<string> s(h); rep(i,h) cin >> s[i];

    int gi, gj;
    queue<P> que;
    vector seen(h, vector(w, inf));
    vector<vp> tp(26);
    vector used(26, false);
    rep(i,h) rep(j,w) {
        if(s[i][j] >= 'a' && s[i][j] <= 'z') {
            tp[s[i][j] - 'a'].push_back({i,j});
        } else if(s[i][j] == 'S') {
            seen[i][j] = 0;
            que.push({i,j});
        } else if(s[i][j] == 'G') {
            gi = i, gj = j;
        }
    }

    assert(!que.empty());

    while(!que.empty()) {
        auto [pi, pj] = que.front(); que.pop();
        dbg(pi,pj);
        if(s[pi][pj] >= 'a' && s[pi][pj] <= 'z' && !used[s[pi][pj] - 'a']) {
            used[s[pi][pj] - 'a'] = true;
            for(auto [ni, nj]: tp[s[pi][pj] - 'a']) {
                if(ni == pi && nj == pj) continue;
                if(seen[ni][nj] != inf) continue;;
                seen[ni][nj] = seen[pi][pj] + 1;
                que.push({ni,nj});
            }
        }
        rep(r, 4) {
            int ni = pi + di[r];
            int nj = pj + dj[r];
            if(ni>=0 && nj>=0 && ni<h && nj<w && seen[ni][nj] == inf && s[ni][nj] != '#') {
                seen[ni][nj] = seen[pi][pj] + 1;
                que.push({ni,nj});
            }
        }
    }
    cout << ((seen[gi][gj] == inf) ? -1 : seen[gi][gj]) << '\n';
}

int main() {
    cin.tie(0);
    ios::sync_with_stdio(false);
    int t=1;
    //cin >> t;
    while(t--)solve();
}


