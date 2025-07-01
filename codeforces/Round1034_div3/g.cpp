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


vll divs(ll n) {
    vll res;
    rep2(i,1,sqrt(n)+2) {
        if(n%i==0) {
            res.emplace_back(i);
            if(n/i != i) res.emplace_back(n/i);
        }
    }
    sort(all(res));
    return res;
}


void solve() {
    int n, m, q; cin >> n >> m >> q;
    auto a = i64_vec_IN(n);

    auto divisors = divs(m);
    int l = divisors.size();

    vector<int> ans(l,0);
    rep(idx,l) {
        rep(i,n-1) {
            int me = a[i]%divisors[idx];
            int ne = a[i+1]%divisors[idx];
            if(me>ne) {
                ans[idx]++;
            }
        }
    }

    rep(_,q) {
        int t; cin >> t;
        if(t==1) {
            int i, x; cin >> i >> x;
            i--;
            
            rep(idx,l) {
                int old = a[i]%divisors[idx];
                int nw = x%divisors[idx];
                if(i>0) {
                    int lo = a[i-1]%divisors[idx];
                    if(lo<=old && lo>nw) {
                        ans[idx]++;
                    } else if(lo>old && lo<=nw) {
                        ans[idx]--;
                    }
                }
                if(i+1<n) {
                    int hi = a[i+1]%divisors[idx];
                    if(old<=hi && nw>hi) {
                        ans[idx]++;
                    } else if(old>hi && nw<=hi) {
                        ans[idx]--;
                    }
                }
            }
            a[i] = x;

        } else {
            int k; cin >> k;
            int x = gcd(k,m);
            int idx = lower_bound(all(divisors), x) - divisors.begin();
            cout << ((ans[idx]<m/x)?"Yes":"No") << '\n';
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
