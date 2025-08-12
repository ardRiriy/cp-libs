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
using P = pair<int,int>;
using vp=vector<pair<ll, ll>>;
using djks=priority_queue<P, vp, greater<P>>;

const int inf=1ll<<30;
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

pair<P, P> move_to(P l, vector<string>& bl, P r, vector<string>& br, int rr) {
    if(rr==0 || rr==2) {
        // 上下
        ll nl = l.first + di[rr];
        if(nl >= 0 && nl < bl.size() && bl[nl][l.second] != '#') l.first = nl;
        
        ll nr = r.first + di[rr];
        if(nr >= 0 && nr < br.size() && br[nr][r.second] != '#') r.first = nr;
    } else {
        ll nl = l.second + dj[rr];
        if(nl >= 0 && nl < bl[0].size() && bl[l.first][nl] != '#') l.second = nl;

        ll nr = r.second + dj[(rr+2)%4];
        if(nr >= 0 && nr < br[0].size() && br[r.first][nr] != '#') r.second = nr;

    }
    return {l, r};
}

vector seen(50, vector(50, vector(50, vector(50, false))));

int solve(){
    int w, h; cin >> w >> h;
    if(w==0) return 1;
    vector<string> l(h), r(h);
    rep(i,h) cin >> l[i] >> r[i];

    pair<int,int> slp, srp;

    rep(i,h) rep(j,w) {
        if(l[i][j] == 'L') {
            l[i][j] = '.';
            slp = {i, j};
        }
        if(r[i][j] == 'R') {
            r[i][j] = '.';
            srp = {i, j};
        }
        rep(ii, h) rep(jj, w) seen[i][j][ii][jj] = false;
    }
    
    seen[slp.first][slp.second][srp.first][srp.second] = true;
    queue<pair<pair<int, int>, pair<int,int>>> que;
    que.push({slp, srp});
    while(!que.empty()) {
        auto [len, rin] = que.front();
        que.pop();

        rep(rr, 4) {
            auto [nlen, nrin] = move_to(len,l,rin,r,rr);
            if(seen[nlen.first][nlen.second][nrin.first][nrin.second]) continue;
            seen[nlen.first][nlen.second][nrin.first][nrin.second] = true;
            ll cnt = 0;
            if(l[nlen.first][nlen.second] == '%')cnt++;
            if(r[nrin.first][nrin.second] == '%')cnt++;
            if(cnt==2) {
                cout << "Yes\n";
                return 0;
            } else if (cnt == 0) {
                que.push({nlen, nrin});
            }
        }
    }
    cout << "No\n";
    return 0;
}

int main() {
    cin.tie(0);
    ios::sync_with_stdio(false);
    while(!solve());
}


