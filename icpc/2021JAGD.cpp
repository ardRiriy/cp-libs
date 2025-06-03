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

class RollingHash {
    static constexpr std::uint64_t MOD = (1ULL << 61) - 1;
    static inline std::uint64_t mod_add(std::uint64_t a, std::uint64_t b) {
        std::uint64_t z = a + b;
        if (z >= MOD) z -= MOD;
        return z;
    }
    static inline std::uint64_t mod_mul(__uint128_t a, __uint128_t b) {
        __uint128_t z = a * b;
        z = (z >> 61) + (z & MOD);
        if (z >= MOD) z -= MOD;
        return static_cast<std::uint64_t>(z);
    }

    static std::uint64_t generate_base() {
        static std::mt19937 rng(std::random_device{}());
        std::uniform_int_distribution<std::uint64_t> dist(2, MOD - 2);
        return dist(rng);
    }

    std::uint64_t base;
    std::vector<std::uint64_t> pow;
    std::vector<std::uint64_t> pref;

public:
    explicit RollingHash(const std::string& s,
                         std::uint64_t base_ = 0)
        : base(base_ ? base_ : generate_base()) {

        if (base <= 1 || base >= MOD)
            throw std::invalid_argument("RollingHash: invalid base");

        std::size_t n = s.size();
        pow.resize(n + 1, 1);
        pref.resize(n + 1, 0);

        for (std::size_t i = 0; i < n; ++i) {
            pow[i + 1]  = mod_mul(pow[i], base);
            std::uint64_t c = static_cast<unsigned char>(s[i]) + 1; // +1 で '\0' 回避
            pref[i + 1] = mod_add(mod_mul(pref[i], base), c);
        }
    }

    std::uint64_t hash(std::size_t l, std::size_t r) const {
        std::uint64_t res = pref[r]
            + MOD
            - mod_mul(pref[l], pow[r - l]);
        if (res >= MOD) res -= MOD;
        return res;
    }

    static std::size_t lcp(const RollingHash& a, std::size_t i,
                           const RollingHash& b, std::size_t j,
                           std::size_t max_len) {
        if (a.base != b.base) throw std::invalid_argument("RollingHash::lcp: base mismatch");

        std::size_t low = 0, high = max_len + 1;
        while (high - low > 1) {
            std::size_t mid = (low + high) >> 1;
            if (a.hash(i, i + mid) == b.hash(j, j + mid))
                low = mid;
            else
                high = mid;
        }
        return low;
    }

    std::uint64_t get_base() const noexcept { return base; }
    static constexpr std::uint64_t mod() noexcept { return MOD; }
};


int solve(){
    string s; cin >> s;
    if(s=="#") return 1;
    int n = s.size();
    int m = s.size() / 2;
    string s1 = "";
    string s2 = "";
    rep(i,m) s1 += s[i];
    rep(i,m) s2 += s[m+n%2+i];
    dbg(s1,s2);

    RollingHash hash1(s1), hash2(s2, hash1.get_base());
    vector<ll> dp(m+1,inf);
    dp[0] = 0;
    rep(i,m) {
        if(dp[i]==inf) continue;
        rep2(x,1,m-i+1) {
            if(hash1.hash(m-x-i,m-i) == hash2.hash(i,i+x)) {
                dbg(i,x);
                chmin(dp[i+x], dp[i] + ((x==1)?0:x*x));
            }
        }
    }
    cout << ((dp[m] == inf)?-1:dp[m]) << '\n';
    return 0;
}

int main() {
    cin.tie(0);
    ios::sync_with_stdio(false);
    while(!solve());
}


