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
    int r, n; cin >> r >> n;
    if(r==0) return 1;
    if(n==0) {
        cout << "0\n";
        return 0;
    }
    // -rを原点とする
    vector<int> h(r, 1<<30);
    map<int,int> mp;
    rep(i,n) {
        int le, ri, hi; cin >> le >> ri >> hi;
        rep2(i,le,ri) {
            if(i+r < 0 || i > r) continue;
            chmax(mp[i],hi);
        }
    }
    rep2(i,-r,r) {
        if(i<0) {
            chmin(h[-i-1], mp[i]);
        } else{
            chmin(h[i], mp[i]);
        }

    }

    double rd = r;
    auto check = [&](const double k) -> bool {
        bool res = true;
        rep(i,r) {
            // x=i
            double x = i;
            double y = (-rd+k) + sqrt(rd*rd - x*x);
            dbg(i,y,h[i]);

            if(y > double(h[i])) res = false;
        }
        return res;
    };

    double ok = 0.0;
    double ng = 40.0;
    while(abs(ok-ng)>1e-7) {
        double mid = (ok+ng)/2;
        dbg(ok,ng,mid);
        if(check(mid)) ok = mid;
        else ng = mid;
    }
    cout << fixed << setprecision(20) << ok << '\n';
    return 0;
}

int main() {
    cin.tie(0);
    ios::sync_with_stdio(false);
    while(!solve());
}


