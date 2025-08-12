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

ll rec(string& s, int& i) {
    assert(s[i++] == '[');
    
    if(s[i] >= '0' && s[i] <= '9') {
        ll res = 0;
        while(s[i] >= '0' && s[i] <= '9') {
            res *= 10;
            res += s[i] - '0';
            i++;
        }
        assert(s[i++] == ']');
        return (res+1)/2;
    }

    vll v;
    while(true) {
        v.emplace_back(rec(s, i));
        if(s[i] == ']') break;
    }
    i++;
    dbg(v);
    sort(all(v));
    ll m = (v.size() + 1) / 2;
    ll res = 0;
    rep(i,m) {
        res += v[i];
    }

    return res;
}

int solve(){
    string s; cin >> s;
    int i = 0;
    cout << rec(s,i) << '\n';
    return 0;
}

int main() {
    cin.tie(0);
    ios::sync_with_stdio(false);
    int t; cin >> t;
    while(t--) solve();
}


