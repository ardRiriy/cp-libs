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

// thanks for: https://algo-logic.info/divisor/#
vector<long long> divisor(long long n) {
    vector<long long> ret;
    for (long long i = 1; i * i <= n; i++) {
        if (n % i == 0) {
            ret.push_back(i);
            if (i * i != n) ret.push_back(n / i);
        }
    }
    sort(ret.begin(), ret.end()); // 昇順に並べる
    return ret;
}

int sol(ll k, vll& a) {
    assert(a[0]==1);

    int n = a.size();
    vector<int> dist(n, -1);
    queue<int> que;
    dist[0] = 0;
    que.push(0);

    while(!que.empty()) {
        int p = que.front(); que.pop();
        rep2(ni,p+1,n) {
            if(dist[ni] != -1) continue;
            if(a[ni]%a[p]==0) {
                ll d = a[ni]/a[p];
                if(d<=k) {
                    dist[ni] = dist[p] + 1;
                    que.push(ni);
                }
            }
        }
    }
    return dist[n-1];
}

void solve() {
    ll x,y,k; cin >> x >> y >> k;
    ll div = gcd(x,y);
    x /= div;
    y /= div;

    auto a =divisor(x);
    int res1 = sol(k,a);
    auto b = divisor(y);
    int res2 = sol(k,b);

    if(res1==-1||res2==-1) {
        cout << "-1\n";
    } else {
        cout << res1+res2 << '\n';
    }
}

int main() {
    cin.tie(0);
    ios::sync_with_stdio(false);
    int t=1;
    cin >> t;
    while(t--)solve();
}


