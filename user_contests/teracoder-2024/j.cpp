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
    int n, m;
    cin >> n >> m;
    string s;
    cin >> s;
    reverse(all(s));

    unsigned long long num = 0;
    int base = 1;
    
    int k;
    // n -> 10
    rep(i, s.length()) {
        if(s[i] >= '0' && s[i] <= '9') {
            k = s[i] - '0';
        } else if(s[i] >= 'A' && s[i] <= 'Z') {
            k = s[i] - 'A' + 10;
        } else {
            k = s[i] - 'a' + 36;
        }

        num += base * k;
        base *= n;
    }


    // 10 -> m
    string ans = "";
    while(num != 0) {
        int k = num % m;
        if(k < 10) {
            ans.push_back(k + '0');
        } else if(k < 36) {
            int l = k - 10;
            ans.push_back(l + 'A');
        } else {
            int l = k - 36;
            ans.push_back(l + 'a');
        }
        num /= m;
    }
    reverse(all(ans));
    if(ans == "") ans = "0";
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