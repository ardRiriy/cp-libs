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
    int n, m, k;
    cin >> n >> m >> k;
    int d1, d2;
    cin >> d1 >> d2;
    long long interval = lcm(d1, d2);

    long long start = max(n, m);
    while ((start - n) % d1 != 0 || (start - m) % d2 != 0) {
        start++;
    }

    int res = start + (k-1) * interval;
    int addr = (res - n) / d1 + 1;
    cout << addr << endl;
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