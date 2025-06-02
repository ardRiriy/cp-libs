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
    int n, m, t, p; cin >> n >> m >> t >> p;
    if(n==0) return 1;

    vector<P> op(t);
    rep(i,t) cin >> op[i].first >> op[i].second;
    
    vvll paper(m, vll(n, 1));
    for(auto [d,c]: op) {
        if(d==1) {
            ll new_n = max(c,n-c);
            vvll new_state(m, vll(new_n, 0));
            rep(i,m) {
                rep(j,c) {
                    new_state[i][j] += paper[i][c-j-1];
                }
            }
            rep(i,m) {
                rep2(j,c,n) {
                    new_state[i][j-c] += paper[i][j];
                }
            }
            paper = new_state;
            n = new_n;
        } else {
            ll new_m = max(c, m-c);
            vvll new_state(new_m, vll(n, 0));
            rep(i,c) {
                rep(j,n) {
                    new_state[i][j] += paper[c-i-1][j];
                }
            }
            rep2(i,c,m) {
                rep(j,n) {
                    new_state[i-c][j] += paper[i][j];
                }
            }
            paper = new_state;
            m = new_m;
        }
    }
    int x,y;
    rep(_, p) {
        cin >> x >> y;
        cout << paper[y][x] << '\n';
    }

    return 0;
}

int main() {
    cin.tie(0);
    ios::sync_with_stdio(false);
    while(!solve());
}

