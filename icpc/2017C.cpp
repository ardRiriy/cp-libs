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

ll dx[] = {1, 0, -1, 0, -1, 1, -1, 1};
ll dy[] = {0, 1, 0, -1, -1, 1, 1, -1};


int solve(){
    int h, w; cin >> h >> w;
    if(h==0) return 1;
    vector<vector<int>> s(h, vector(w, 0));
    rep(i,h) rep(j,w) cin >> s[i][j];

    auto c = [&](const int i, const int di, const int j, const int dj) -> int {
        int mn = 1<<30;
        rep2(y,i,i+di) {
            rep2(x,j,j+dj) {
                if(x==j || y==i || x+1==j+dj || y+1==i+di) {
                    chmin(mn, s[y][x]);
                }
            }
        }
        dbg(i,i+di,j,j+dj);
        dbg(mn);

        int sum = 0;
        rep2(y, i, i+di) {
            rep2(x,j, j+dj) {
                if(x==j || y==i || x+1==j+dj || y+1==i+di) {
                    continue;
                }
                if(mn<=s[y][x]) return -1;
                dbg(mn,s[y][x]);
                sum += mn-s[y][x];
            }
        }
        return sum;
    };
    int ans = 0;
    rep(i,h) rep(j,w) rep2(di,3,h+1) rep2(dj,3,w+1) {
        if(di+i>h || dj+j>w) continue;
        chmax(ans,c(i,di,j,dj));
    }
    cout << ans << '\n';
    return 0;
}

int main() {
    cin.tie(0);
    ios::sync_with_stdio(false);
    while(!solve());
}


