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

template<int MOD>
struct ModInt {
    int value;

    ModInt() : value(0) {}
    ModInt(int value) : value(value % MOD) {
        if (this->value < 0) this->value += MOD;
    }

    ModInt operator+(const ModInt& other) const {
        return ModInt((value + other.value) % MOD);
    }

    ModInt operator-(const ModInt& other) const {
        return ModInt((value - other.value + MOD) % MOD);
    }

    ModInt operator*(const ModInt& other) const {
        return ModInt((1LL * value * other.value) % MOD);
    }

    ModInt operator/(const ModInt& other) const {
        return *this * other.inv();
    }

    ModInt inv() const {
        int a = value, b = MOD, u = 1, v = 0;
        while (b) {
            int t = a / b;
            a -= t * b; std::swap(a, b);
            u -= t * v; std::swap(u, v);
        }
        return ModInt(u);
    }
};

const ll modulo = 1000000009;
const ll base = 411;
using mint = ModInt<modulo>;
using S = mint;
S op(S a, S b) { return a+b; }
S e() { return 0; }

void solve() {
    int n, q; cin >> n >> q;
    string s; cin >> s;
    string revs=s; reverse(all(revs));
    segtree<S, op, e> seg(n), rseg(n);

    vector<mint> b(n);
    b[n-1] = 1;
    per(i, n-1) {
        b[i] = b[i+1] * base;
    } 

    rep(i,n) {
        seg.set(i, mint(int(s[i]))*b[i]);
        rseg.set(i, mint(int(revs[i]))*b[i]);
    }

    rep(_, q) {
        int t; cin >> t;
        if(t==1) {
            int x; char c; cin >> x >> c;
            x--;
            seg.set(x, mint(int(c))*b[x]);
            rseg.set(n-x-1, mint(int(c))*b[n-x-1]);
        } else {
            int l, r; cin >> l >> r;

            l--;
            int m = (r+l)/2;
            int m2 = ((r-l)%2==0)?n-m:n-m-1;

            mint h1 = seg.prod(l, m) / b[m];
            mint h2 = rseg.prod(n-r, m2) / b[m2];
            cout << ((h1.value==h2.value) ? "Yes\n" : "No\n");
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


