#include <bits/stdc++.h>
#define int long long
#define rep(i, n) for (int i = 0; i < (int)(n); i++)
#define revrep(i, n) for (int i = (int)(n); i >= 0; i--)
#define itrep(itr, stl) for (auto itr = stl.begin(); itr != stl.end(); itr++)
#define Vec2D(type, n, m, val) vector<vector<type>>(n, vector<type>(m, val))
#define print(x) cout << x << endl
#define all(a) a.begin(), a.end()
const int INF = LLONG_MAX;
const int N_INF = LLONG_MIN;
using namespace std;

////////////////////////////////////////

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


bool isKaibun(string s) {
    string t = s;
    reverse(all(t));
    return s == t;
}

string add_c(int k, string s, char c) {
    s.push_back(c);
    if(s.length() > k) {
        s.erase(0, 1);
    }
    return s;
}

signed main() {
    std::cout << std::fixed;
    std::cout << std::setprecision(20);
    std::cin.tie(0)->sync_with_stdio(0);
    using mint = ModInt<998244353>;
    int n, k;
    string s;
    cin >> n >> k >> s;
    
    map<string, mint> mp;
    vector<vector<map<string, mint>>> dp = vector(2, vector<map<string, mint>>(k+1, mp));
    int now = 0; 
    dp[now][0][""] = 1;

    rep(idx, n) {
        int nxt = 1 - now;
        revrep(i, k) {
            itrep(itr, dp[now][i]) {
                rep(j, 2) {
                    char c1 = 'A' + j;
                    char c2 = 'A' + (1-j);
                    if(s[idx] != c1) {
                        string s = add_c(k, itr->first, c2);
                        if(!isKaibun(s) || s.length() < k) {
                            dp[nxt][min(i+1, k)][s] = dp[nxt][min(i+1, k)][s] + itr->second;
                        }
                    }
                }
            }
        }

        dp[now] = vector<map<string, mint>>(k+1, mp);
        now = nxt;
    }

    mint ans = 0;
    itrep(itr, dp[now][k]) {
        ans = ans + itr->second; 
    }
    cout << ans.value << endl;
}

