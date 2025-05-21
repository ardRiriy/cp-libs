
def judge(m, y, c, r):
    money = m
    for _ in range(y):
        money -= c
        if money >= m:
            return True
        if(money < 0): return False
        money = money * (100+r) // 100
    return True

while True:
    y, c, r = map(int, input().split())
    if y==0: break

    ok = c*y
    ng = -1
    while abs(ok-ng)>1:
        mid = (ok+ng)//2
        if judge(mid,y,c,r):
            ok=mid
        else:
            ng=mid

    print(ok)

"""
手で書き換えたヨ

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

bool judge(ll m, ll y, ll c, ll r) {
    rep(_, y) {
        m -= c;
        if(m<0) return false;
        dbg(m);
        m = m * (100+r) / 100;
    }

    return true;
}


int solve(){
    ll y, c, r; cin >> y >> c >> r;
    if(y==0) return 1;

    ll ok = c*y;
    ll ng = -1;

    while(abs(ok-ng)>1) {
        ll mid = (ok+ng)>>1;
        if(judge(mid, y, c, r)) {
            ok = mid;
        } else {
            ng = mid;
        }
    }
    cout << ok << '\n';
    return 0;
}

int main() {
    cin.tie(0);
    ios::sync_with_stdio(false);
    while(!solve());
}
"""