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


using mint = ModInt<998244353>;
void solve() {
    int h, k; cin >> h >> k;
    // dp i, j := 家がi本立っており、体力がjであるような状態に到達する確率
    vector<vector<mint>> dp(k+1, vector<mint>(h+1, 0));
    dp[0][h] = 1;

    rep(i, k) {
        dp[i][1] = dp[i][1] + dp[i][0];
        rep2(j, 1, h) {
            dp[i+1][j-1] = dp[i+1][j-1] + dp[i][j] * j / h;
            dp[i][j+1] = dp[i][j+1] + dp[i][j] * (h - j) / h;
        }
        dp[i+1][h-1] = dp[i+1][h-1] + dp[i][h];
    }

    mint ans = 0;

    rep(i, h+1) {
        // K + i - (H-K)本を消費
        // 気持ち: 家を建てるのにK本、合計で体力Kを消費していて、それがiまで回復している
        ans = ans + dp[k][i] * (k + i - (h-k)); 
    }

    cout << ans.value << '\n';

}

int main() {
    cin.tie(0);
    ios::sync_with_stdio(false);
    int t=1;
    //cin >> t;
    while(t--)solve();
}