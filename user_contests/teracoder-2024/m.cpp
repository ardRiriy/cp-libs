#include <bits/stdc++.h>
#define int long long
#define rep(i, n) for (int i = 0; i < (int)(n); i++)
#define revrep(i, n) for (int i = (int)(n); i >= 0; i--)
#define itrep(itr, stl) for (auto itr = stl.begin(); itr != stl.end(); itr++)
#define Vec2D(type, n, m, val) vector<vector<type>>(n, vector<type>(m, val))
#define print(x) cout << x << endl
#define all(a) a.begin(), a.end()
using namespace std;

bool chmin(int &a, int b) {
    if (a > b) {
        a = b;
        return true;
    }
    return false;
}
bool chmax(int &a, int b) {
    if (a < b) {
        a = b;
        return true;
    }
    return false;
}

int power(int x, int n) {
    int result = 1;
    while (n > 0) {
        if ((n & 1) == 1) {
            result *= x;
        }
        x *= x;
        n >>= 1;
    }
    return result;
} 

void solve() {
    // hogehoge
    int n;
    cin >> n;

    int MOD = 998244353;
    int ans = 0;
    for(int i = 2; i < n; i ++) {
        int k = n - i;
        int x = (i * (i+1) / 2) % MOD;
        ans += (i-1) * ((n * (n+1) / 2) % MOD - x + MOD) % MOD - (k * i) % MOD;
        ans = (ans + MOD) % MOD;
    }
    cout << ans << endl;
}

signed main() {
    std::cout << std::fixed;
    std::cout << std::setprecision(20);
    std::cin.tie(0)->sync_with_stdio(0);
    int times = 1;
    // cin >> times;
    while (times--) solve();
    return 0;
}