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
    int n; cin >> n;
    auto a = i64_vec_IN(n);
    sort(all(a));

    vector<int> cnt(n+1);
    rep(i,n) cnt[a[i]]++;

    vector<int> s(n+1, 0), t(n+1,0);
    rep(i,n+1) {
        if(i>0) s[i] = s[i-1];
        s[i] += cnt[i]-1; // cnt[i] == 0なら壊れるけど気にしない☆
    }

    per(i,n) {
        t[i] = t[i+1] + cnt[i+1];
    }

    bool out = false;
    vector<int> imos(n+1, 0);
    rep(i,n+1) {
        if(out || cnt[i]==0) {
            if(!out)imos[0]++;
            out = true;
            continue;
        }

        imos[cnt[i]]++;
        imos[s[i]+t[i]+1]--;
    }

    int cur = 0;
    rep(i,n+1) {
        cur += imos[i];
        cout << cur << " \n"[i==n];
    }
    return;
}

int main() {
    cin.tie(0);
    ios::sync_with_stdio(false);
    int t=1;
    cin >> t;
    while(t--)solve();
}


