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
const double PI = acos(-1);

#define rep(i,n) for (ll i=0;i<n;++i)
#define per(i,n) for(ll i=n-1;i>=0;--i)
#define rep2(i,a,n) for (ll i=a;i<n;++i)
#define per2(i,a,n) for (ll i=n-1;i>=a;--i)


template<class T>bool chmax(T &a, const T &b) { if (a<b) { a=b; return true; } return false; }
template<class T>bool chmin(T &a, const T &b) { if (b<a) { a=b; return true; } return false; }

ll dx[] = {1, 0, -1, 0, -1, 1, -1, 1};
ll dy[] = {0, 1, 0, -1, -1, 1, 1, -1};

using pos = pair<int,int>;

int n, m;
vector<vector<int>> a;
vector<vector<ll>> fromStart, fromEnd;

void dfs(pos cur, int left, ll val, int rid, pos start) {
    // rid = 0: 右/下, 1: 左/上
    left--;
    ll currentVal = val;


    if(!(rid==1&&left==n-1)) {
        currentVal = (currentVal * 10 + a[cur.first][cur.second]) % m;
    }

    if(left==0) {
        if(rid==0) {
            fromStart[cur.second].emplace_back(currentVal);
        } else {
            fromEnd[start.second].emplace_back(currentVal);
        }
        return;
    }

    pos nxt = cur;
    nxt.second++;
    if(nxt.first<n&&nxt.second<n) dfs(nxt,left,currentVal,rid,start);

    nxt = cur;
    nxt.first++;
    if(nxt.first<n&&nxt.second<n) dfs(nxt,left,currentVal,rid,start);

    return;
}

void solve() {
    cin >> n >> m;

    a.resize(n);
    fromStart.resize(n);
    fromEnd.resize(n);
    rep(i,n) a[i].resize(n);

    rep(i,n) rep(j,n) cin >> a[i][j];

    dfs({0,0}, n, 0, 0, {0,0});
    rep(j,n) dfs({n-j-1,j}, n, 0, 1, {n-j-1,j});

    rep(i,n) sort(all(fromEnd[i]));

    ll muler = 1;
    rep(i,n-1) muler = (muler * 10) % m;

    rep(i,n) for(ll& val: fromStart[i]) val = (val * muler) % m;

    ll ans = 0;
    rep(i,n) {
        if(fromEnd[i].empty()) continue;
        for(ll a: fromStart[i]) {
            int ok = -1;
            int ng = fromEnd[i].size();
            while(abs(ng-ok)>1){
                int mid = (ok+ng)>>1;
                if(a+fromEnd[i][mid]<m) {
                    ok = mid;
                } else {
                    ng = mid;
                }
            }

            if(ok==-1) {
                ok = fromEnd[i].size()-1;
            }

            chmax(ans, (a+fromEnd[i][ok])%m);
        }
    }
    cout << ans << '\n';
}

int main() {
    cin.tie(0);
    ios::sync_with_stdio(false);
    int t=1;
    //cin >> t;
    while(t--)solve();
}


