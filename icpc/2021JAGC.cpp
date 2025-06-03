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

ll di[] = {0, 1, 1};
ll dj[] = {1, 0, 1};

ll ans = 0;
int h, w;

map<ll, ll> memo;
ll rec(vector<string>& s, ll state, int i, int j) {
    if(i >= h) {
        dbg(state);
        dbg(__popcount(state));
        if(__popcount(state) == h*w) return 1; 
        else return 0;
    }

    if(memo.contains(state)) return memo[state];
    ll res = 0;
    int ni = (j==w-1) ? i+1 : i;
    int nj = (j==w-1) ? 0 : j+1;

    ll idx = i*w+j;
    if((state>>idx)&1LL==1LL) {
        res += rec(s,state,ni,nj);
        bool isOk =  true;
        rep(r,3) {
            int ri = i + di[r];
            int rj = j + dj[r];
            if(!(ri<h && rj<w && ((state>>(ri*w+rj))&1)!=1)) isOk = false;
        }
        if(isOk) {
            ll next_state = state;
            next_state = next_state | (1ll << idx);
            rep(r,3) {
                int ri = i + di[r];
                int rj = j + dj[r];
                next_state = next_state | (1ll << (ri*w+rj));
            }
            // cerr << "\n========\n";
            // rep(i,h) rep(j,w) cerr << ((((state>>(i*w+j))&1)==1)?"1":"0") << " \n"[j+1==w];
            // cerr << "\n----\n";
            // rep(i,h) rep(j,w) cerr << ((((next_state>>(i*w+j))&1)==1)?"1":"0") << " \n"[j+1==w];
            res += rec(s,next_state,ni,nj);
        }
    } else {
        bool isLeastOne = false;
        rep(exclude, 3) {
            // exclude以外に配置可能か
            isLeastOne = true;
            bool isOk = true;
            rep(r,3) {
                if(r==exclude)continue;
                int ri = i + di[r];
                int rj = j + dj[r];
                if(!(ri<h && rj<w && ((state>>(ri*w+rj))&1)!=1)) isOk = false;
            }
            if(!isOk) continue;
            ll next_state = state;
            next_state = next_state | (1ll << idx);
            rep(r,3) {
                if(r==exclude)continue;
                int ri = i + di[r];
                int rj = j + dj[r];
                next_state = next_state | (1ll << (ri*w+rj));
            }
            // cerr << "\n========\n";
            // rep(i,h) rep(j,w) cerr << ((((state>>(i*w+j))&1)==1)?"1":"0") << " \n"[j+1==w];
            // cerr << "\n----\n";
            // rep(i,h) rep(j,w) cerr << ((((next_state>>(i*w+j))&1)==1)?"1":"0") << " \n"[j+1==w];
            res += rec(s,next_state,ni,nj);
        }
        if(!isLeastOne) return memo[state] = 0;
    }

    return memo[state] = res;
}

int solve(){
    cin >> h >> w;
    if(h==0) return 1;

    memo.clear();
    ans = 0;
    
    vector<string> s(h); rep(i,h) cin >> s[i];
    ll state = 0;
    rep(i,h) rep(j,w) if(s[i][j] == '#') state = state | (1ll << (i*w+j));
    cout << rec(s,state,0,0) << '\n';
    return 0;
}

int main() {
    cin.tie(0);
    ios::sync_with_stdio(false);
    while(!solve());
}


