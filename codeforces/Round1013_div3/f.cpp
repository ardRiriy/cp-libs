#include <bits/stdc++.h>
using namespace std;

#define all(v) v.begin(),v.end()
#define resort(v) sort(v.rbegin(),v.rend())
using ll = long long;
using ull = unsigned long long;
using vll=vector<ll>;
using vvll = vector<vector<ll>>;
using P = pair<ll,ll>;
using vp=vector<pair<ll, ll>>;

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
    int n, m, d; cin >> n >> m >> d;
    vector<string> board(n);
    rep(i, n) cin >> board[i];

    vector<vector<vector<mint>>> dp(n, vector<vector<mint>>(m, vector<mint>(2, 0)));
    vector<vector<vector<mint>>> csum(n, vector<vector<mint>>(m+1, vector<mint>(2, 0)));
    rep(i, m) {
        if(board[n-1][i] == 'X') dp[n-1][i][0] = 1;
    }

    per(i, n) {
        if(i != n-1) {
            // 一つ下のレベルからもらう
            rep(j, m) {
                if(board[i][j] == '#') continue;
                dp[i][j][0] = csum[i+1][min(j+d, (ll)m)][0] - csum[i+1][max(j-d+1, 0LL)][0];
                dp[i][j][0] = dp[i][j][0] + csum[i+1][min(j+d, (ll)m)][1] - csum[i+1][max(j-d+1, 0LL)][1];
            }
        }

        rep(j, m) {
            csum[i][j+1][0] = csum[i][j][0] + dp[i][j][0];
        }

        rep(j, m) {
            // 同じレベルからもらう
            if(board[i][j] == '#') continue;
            dp[i][j][1] = csum[i][min(j+d+1, (ll)m)][0] - csum[i][max(j-d, 0LL)][0];
            dp[i][j][1] = dp[i][j][1] - dp[i][j][0];
        }

        rep(j, m) {
            csum[i][j+1][1] = csum[i][j][1] + dp[i][j][1];
        }

    }

    mint sum = 0;
    rep(i, m) sum = sum + dp[0][i][0] + dp[0][i][1];
    cout << sum.value << '\n'; 
}

int main() {
    cin.tie(0);
    ios::sync_with_stdio(false);
    int t;
    cin >> t;
    while(t--)solve();
}