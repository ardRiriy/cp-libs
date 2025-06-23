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

#define rep(i,n) for (int i=0;i<(n);++i)
#define per(i,n) for(ll i=(n)-1;i>=0;--i)
#define rep2(i,a,n) for (ll i=(a);i<(n);++i)
#define per2(i,a,n) for (ll i=(n)-1;i>=(a);--i)


template<class T>bool chmax(T &a, const T &b) { if (a<b) { a=b; return true; } return false; }
template<class T>bool chmin(T &a, const T &b) { if (b<a) { a=b; return true; } return false; }

ll dx[] = {1, 0, -1, 0, -1, 1, -1, 1};
ll dy[] = {0, 1, 0, -1, -1, 1, 1, -1};

void solve() {
    int h, w, k; cin >> h >> w >> k;

    vector<string> s(h);
    rep(i,h) cin >> s[i];
    vvll v(h, vll(w, 0));
    rep(i,h) rep(j,w) if(s[i][j] == 'g') v[i][j] = 1;
    vvll csum(h+1, vll(w+1, 0));
    rep(i,h) {
        rep(j,w) {
            csum[i+1][j+1] = csum[i][j+1] + csum[i+1][j] - csum[i][j] + v[i][j];
        }
    }

    ll al = 0;
    ll cnt = inf;
    rep(i,h) {
        rep(j,w) {
            if(s[i][j] == 'g') al++;
            if(s[i][j] != '.') continue;
            dbg(min(h,i+k));
            dbg(min(w,j+k));
            dbg(max(j-k+1,0));
            dbg(max(i-k+1,0));
            ll c = csum[min(h,i+k)][min(w,j+k)] - csum[min(h,i+k)][max(j-k+1,0)]
                - csum[max(i-k+1,0)][min(w,j+k)] + csum[max(i-k+1,0)][max(j-k+1,0)];
            // rep2(ki, max(i-k+1,0), min(h,i+k)) {
            //     rep2(kj, max(j-k+1, 0), min(w, j+k)) {
            //         if(s[ki][kj] == 'g') c++;
            //     }
            // }
            chmin(cnt,c);
        }
    }
    cout << al-cnt << '\n';
}

int main() {
    cin.tie(0);
    ios::sync_with_stdio(false);
    int t=1;
    cin >> t;
    while(t--)solve();
}


