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

int MOD = 998244353;
int power(int t, int n) {
    int x = t % MOD;
    int result = 1;
    while (n > 0) {
        if ((n & 1) == 1) {
            result *= x;
            result %= MOD;
        }
        x *= x;
        x %= MOD;
        n >>= 1;
    }
    return result;
} 

void solve() {
    // hogehoge
    int a, n;
    cin >> a >> n;
    int sum = n * (n+1) / 2;
    int sum2 = (a-1) * a / 2;
    sum = power(sum, 2);
    sum2 = power(sum2, 2);

    cout << (sum - sum2 + MOD) % MOD << endl;
    

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