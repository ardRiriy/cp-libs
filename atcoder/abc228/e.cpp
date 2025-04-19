#include <bits/stdc++.h>
#include "input.hpp"
using namespace std;

// using namespace atcoder;

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

/* thank you!: https://trap.jp/post/1444/ */
constexpr uint32_t totient(uint32_t x){
    uint32_t ans = x;
    for(uint32_t i = 2; i * i <= x; i++) if(x % i == 0){
        ans /= i;
        ans *= i - 1;
        do x /= i; while(x % i == 0);
    }
    if(x != 1){
        ans /= x;
        ans *= x - 1;
    }
    return ans;
}
template<uint32_t P> struct Modint{
    static_assert(P < 0x80000000, "P must be smaller than 2^31");
    uint32_t a;
    Modint<totient(P)> b;
private:
    static uint32_t mod(uint64_t x){
        if(x < P * 2) return uint32_t(x);
        return uint32_t(x % P) + P;
    }
    static uint32_t mul(uint32_t a, uint32_t b){
        return mod(uint64_t(a) * b);
    }
    static uint32_t pow(uint32_t a, uint32_t b){
        uint32_t ans = 1;
        while(b){
            if(b & 1) ans = mul(ans, a);
            a = mul(a, a);
            b >>= 1;
        }
        return ans;
    }
public:
    Modint(uint64_t x): a(mod(x)), b(x){}
    Modint(uint32_t a, Modint<totient(P)> b): a(a), b(b){}
    uint32_t val() const {
        if(a < P) return a;
        return a - P;
    }
    Modint pow(const Modint& other) const {
        return {pow(a, other.b.a), b.pow(other.b)};
    };
};
template<> struct Modint<1>{
    uint32_t a;
    Modint(uint64_t x): a(bool(x)){}
    uint32_t val() const {
        return 0;
    }
    Modint pow(const Modint& other) const {
        return {a || !other.a};
    }
};

void solve() {
    using mint = Modint<998244353>;
    ll n, k, m; cin >> n >> k >> m;
    mint nn = n;
    mint kk = k;
    mint mm = m;
    cout << mm.pow(kk.pow(nn)).val() << '\n';
}

int main() {
    cin.tie(0);
    ios::sync_with_stdio(false);
    int t=1;
    //cin >> t;
    while(t--)solve();
}


