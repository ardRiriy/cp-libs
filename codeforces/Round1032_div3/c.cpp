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
    // 最大値-1が達成できるかどうかを考える
    // 最大値を調べる
    // - 1 / 2個の場合: 達成可能
    // それ以上の場合: 候補の点を全部持つ

    int n, m; cin >> n >> m;
    vvll a(n, vll(m)); rep(i,n) rep(j,m) cin >> a[i][j];

    ll max_val = -1;
    P pos = {-1, -1}, pos2 = {-1,-1};
    rep(i,n) {
        rep(j,m) {
            if(max_val == a[i][j]) {
                if(pos.first != i && pos.second != j) pos2 = {i,j};
            } else {
                if(chmax(max_val, a[i][j])) {
                    pos = {i,j};
                    pos2 = {-1,-1};
                }
            }
        }
    }

    if(pos2.first==-1) {
        cout << max_val-1 << '\n';
        return;
    }

    P cand1 = {pos.first, pos2.second}, cand2 = {pos2.first, pos.second};
    bool b1 = true, b2 = true;
    rep(i,n) rep(j,m) {
        if(max_val == a[i][j]) {
            b1 = b1 && (cand1.first == i || cand1.second == j);
            b2 = b2 && (cand2.first == i || cand2.second == j);
        }
    }
    cout << ((b1||b2)?max_val-1:max_val) << '\n';
}

int main() {
    cin.tie(0);
    ios::sync_with_stdio(false);
    int t=1;
    cin >> t;
    while(t--)solve();
}


