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
    dbg("========");

    int n; cin >> n;
    auto a = i64_vec_IN(n);
    ll start = a[0];
    ll end = a[n-1];
    if(start*2>=end) {
        cout << "2\n";
        return;
    }

    vector<int> st;
    rep2(i,1,n-1) {
        st.emplace_back(a[i]);
    }
    sort(all(st));

    ll cur = start;
    ll cnt=2;
    while(true) {
        // cur*2以下で最大を取る
        ll ok = -1;
        ll ng = st.size();
        while(abs(ok-ng)>1) {
            ll mid = (ok+ng)>>1;
            if(st[mid]<=cur*2) ok = mid;
            else ng = mid;
        }
        dbg(cur,ok);
        if(ok==-1 || st[ok]==cur) {
            cout << "-1\n";
            return;
        }

        cur=st[ok];
        cnt++;
        if(cur*2>=end) {
            cout << cnt << '\n';
            return;
        }
    }
}

int main() {
    cin.tie(0);
    ios::sync_with_stdio(false);
    int t=1;
    cin >> t;
    while(t--)solve();
}


