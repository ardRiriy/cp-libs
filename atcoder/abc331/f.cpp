#include <bits/stdc++.h>
#include <atcoder/all>

using namespace std;
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

#define rep(i,n) for (ll i=0;i<n;++i)
#define per(i,n) for(ll i=n-1;i>=0;--i)
#define rep2(i,a,n) for (ll i=a;i<n;++i)
#define per2(i,a,n) for (ll i=n-1;i>=a;--i)


template<class T>bool chmax(T &a, const T &b) { if (a<b) { a=b; return true; } return false; }
template<class T>bool chmin(T &a, const T &b) { if (b<a) { a=b; return true; } return false; }

ll dx[] = {1, 0, -1, 0, -1, 1, -1, 1};
ll dy[] = {0, 1, 0, -1, -1, 1, 1, -1};
using u128 = __uint128_t;

const u128 modulo = (1LL << 62) -1;
using S = u128;
S op(S a, S b){ return (a+b)%modulo; };
S e(){ return 0; }


u128 hs(segtree<S, op, e>& seg, vector<u128>& power, int l, int r) {
    u128 val1 = seg.prod(0, r);
    u128 val2 = seg.prod(0, l);
    return (val1 - power[r-l] * val2) % modulo;
}

void solve() {
    random_device seed_gen;
    mt19937 engine(seed_gen());
    int base = engine();
    int n, q; cin >> n >> q;

    vector<u128> b(n, 1);
    per2(i, 1, n) {
        b[i-1] = (b[i] * base) % modulo;
    }

    vector<u128> power(n, 1);
    rep(i, n) {
        power[i+1] = (power[i] * base) % modulo;
    }

    string s, rev_s; cin >> s;
    rev_s = s;
    reverse(all(s));

    segtree<S, op, e> seg(n), rseg(n);
    rep(i, n) {
        seg.set(i, (s[i] * b[i]) % modulo);
        rseg.set(i, (rev_s[i] * b[i]) % modulo);
    }

    rep(_, q) {
        int t; cin >> t;
        if(t==1) {
            int x; cin >> x;
            char c; cin >> c;
            seg.set(x-1, (b[x-1]*c)%modulo);
            rseg.set(n-(x-1), (b[x-1]*c)%modulo);
        } else {
            int l, r; cin >> l >> r;
            l--; r;
            if(l==r) {
                cout << "Yes\n";
                continue;
            }
            int m = (r+l)/2;
            u128 h1 = hs(seg, power, l, m);
            u128 h2 = hs(rseg, power, n-r, n-m);
            
            cout << ((h1==h2)?"Yes\n":"No\n");
        }
    }

}

int main() {
    cin.tie(0);
    ios::sync_with_stdio(false);
    int t=1;
    //cin >> t;
    while(t--)solve();
}


