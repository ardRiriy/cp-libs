#include <bits/stdc++.h>
#include "input.hpp"

using namespace std;

#include "atcoder/segtree.hpp"
using namespace atcoder;

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

using S = ll;
S op(S a, S b) { return a + b; }
S e() { return 0; }

void solve() {
    const int n = 3e6;
    int q; cin >> q;


    vector<S> v(n,1);
    bitset<n> seen;
    segtree<S,op,e> seg(v);
    seg.set(0,0);

    ll a, b;
    rep(_,q){
        cin >> a >> b;
        if(a<n&&!seen[a]) {
            ll k = a;
            while(k<n){
                seen[k] = true;
                seg.set(k,0);
                k += a;
            }
        }
        
        int ng = -1;
        int ok = n;
        while(ok-ng>1){
            int mid = (ok+ng)>>1;
            if(seg.prod(0,mid)>=b){
                ok=mid;
            } else {
                ng=mid;
            }
        }
        cout << ok - 1 << '\n';
    }
}

int main() {
    cin.tie(0);
    ios::sync_with_stdio(false);
    int t=1;
    //cin >> t;
    while(t--)solve();
}


