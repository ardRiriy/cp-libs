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
    dbg("=============");
    int n, k; cin >> n >> k;
    auto a = i64_vec_IN(n);

    vll front_csum(n+1), back_csum(n+1);

    rep(i,n) {
        ll cnt = 1;
        ll prev = ((i==0)?-1:a[i-1]);
        ll t = a[i];
        while(t%2==0) {
            if(t/2==prev) break;
            t /= 2;
            cnt *= 2;
        }
        front_csum[i+1] = front_csum[i] + cnt;
    }

    rep(i,n) {
        ll cnt = 1;
        ll prev = ((i==0)?-1:a[n-i]);
        ll t = a[n-i-1];
        while(t%2==0) {
            if(t/2==prev) break;
            t /= 2;
            cnt *= 2;
        }
        back_csum[i+1] = back_csum[i] + cnt;
    }

    ll maximam_op_num = n;
    rep(i,n+1) {
        ll res1 = front_csum[n] - front_csum[i];
        ll res2 = back_csum[n] - back_csum[n-i];
        dbg(res1);
        dbg(res2);
        chmax(maximam_op_num, res1 + res2);
    }
    cout << ((maximam_op_num >= k && k >= n)? "Yes\n": "No\n");

}

int main() {
    cin.tie(0);
    ios::sync_with_stdio(false);
    int t=1;
    cin >> t;
    while(t--)solve();
}


