#include <bits/stdc++.h>
#include "input.hpp"

using namespace std;

// using namespace atcoder;

#ifdef ADRY
#include <dbg.h>
#else
// DO NOTHING
#define dbg(x,...)
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
    int w, h; cin >> w >> h;
    if(w==0) return 1;
    int k = h+w-1;
    vector<vector<int>> board(h, vector<int>(w, inf));
    int x, y;
    rep(i,k) {
        cin >> x >> y;
        x--;y--;
        cin >> board[y][x];
    }

    
    // 0: 横
    vector<vector<int>> num(2, vector<int>(max(h,w), inf));
    queue<pair<int,int>> que; // (index, yoko or tate)
    num[0][0] = 0;
    que.push({0,0}); 

    while(!que.empty()) {
        auto [p, cur] = que.front();que.pop();
        dbg(p, cur);
        if(cur==0) {
            rep(i,h) {
                if(board[i][p] == inf) continue;
                if(num[1-cur][i] != inf) continue;
                num[1-cur][i] = board[i][p] - num[cur][p];
                que.push({i,1-cur});
            }
        } else {
            rep(i,w) {
                if(board[p][i] == inf) continue;
                if(num[1-cur][i] != inf) continue;
                num[1-cur][i] = board[p][i] - num[cur][p];
                que.push({i,1-cur});
            }
        }
    }
    
    bool isAllNonInf = true;
    rep(i,w) isAllNonInf = isAllNonInf && num[0][i] != inf;
    rep(i,h) isAllNonInf = isAllNonInf && num[1][i] != inf;
    cout << ((isAllNonInf) ? "YES":"NO") << '\n';
    return 0;

}

int main() {
    cin.tie(0);
    ios::sync_with_stdio(false);
    while(!solve());
}


