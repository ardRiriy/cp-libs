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
    vector<string> s(n);
    set<string> set;
    rep(i, n) cin >> s[i];

    string revs;
    int p[2] = {0, 0};
    rep(i, n) {
        if(i!=0) {
            if (s[i-1][s[i-1].size()-1] != s[i][0] || set.find(s[i]) != set.end()) {
                if(p[0] == p[1]) cout << "draw\n-1" << endl;
                else if(p[0] > p[1]) cout << "Terako\n" << p[0] << endl;
                else cout << "TerakoA\n" << p[1] << endl;
                return;
            }
        }
        revs = s[i];
        reverse(all(revs));
        if(revs == s[i]) {
            p[i%2] += 2;
        } else {
            p[i%2] += 1;
        }
        set.insert(s[i]);
    }
    if(p[0] == p[1]) cout << "draw\n-1" << endl;
    else if(p[0] > p[1]) cout << "Terako\n" << p[0] << endl;
    else cout << "TerakoA\n" << p[1] << endl;
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